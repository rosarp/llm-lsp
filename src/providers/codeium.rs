use super::{
    codeium_types,
    llm_api::{CompletionRequest, LlmClientApi, LlmState},
};
use async_lsp::{
    lsp_types::{
        CompletionItem, CompletionItemKind, CompletionResponse, CompletionTextEdit, Position,
        Range, TextEdit,
    },
    ResponseError,
};
use futures::future::BoxFuture;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, AUTHORIZATION, CONNECTION, CONTENT_TYPE},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

impl LlmClientApi for LlmState {
    fn new(api_key: &str, session_id: &str) -> LlmState {
        let auth_url = "https://web-backend.codeium.com/exa.language_server_pb.LanguageServerService/GetCompletions".to_owned();
        let mut headers = HeaderMap::with_capacity(4);
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(
            AUTHORIZATION,
            format!("Basic {}-{}", api_key, session_id)
                .as_str()
                .parse()
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        let client = reqwest::Client::builder()
            .default_headers(headers.clone())
            .build()
            .unwrap();
        LlmState {
            auth_url,
            api_key: api_key.to_owned(),
            session_id: session_id.to_owned(),
            client,
        }
    }

    fn chat(&self) {}

    fn completion(
        &self,
        request: CompletionRequest,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, ResponseError>> {
        let language = codeium_types::LANGUAGE_MAP
            .get(&request.language_id)
            .unwrap_or(&0usize)
            .to_owned();
        let mut cursor_offset = 0usize;
        let lines_len = request
            .contents
            .split("\n")
            .map(|line| line.len())
            .collect::<Vec<usize>>();
        for i in 0..request.position_line {
            if let Some(&len) = lines_len.get(i as usize) {
                cursor_offset += len + 1;
            }
        }
        cursor_offset += request.position_char as usize;
        // The editor name needs to be known by codeium
        // The extensionVersion needs to a recent one, so codeium accepts it
        let request_body = CodeiumRequest {
            metadata: Metadata {
                ide_name: "web".to_owned(),
                ide_version: "unknown".to_owned(),
                extension_version: "0.1.0".to_owned(),
                extension_name: "llm-lsp".to_owned(),
                api_key: self.api_key.clone(),
                session_id: self.session_id.clone(),
            },
            document: Document {
                editor_language: request.language_id,
                language,
                cursor_offset,
                line_ending: "\n".to_owned(),
                absolute_path: request.filepath.clone(),
                relative_path: request.filepath.clone(),
                text: request.contents,
            },
            editor_options: EditorOptions {
                tab_size: 2,
                insert_spaces: true,
            },
            other_documents: vec![],
        };
        let send = self
            .client
            .post(self.auth_url.to_owned())
            .json(&request_body)
            .send();
        Box::pin(async move {
            match send.await {
                Ok(response) => {
                    let status = response.status();
                    let completion_response = match status {
                        StatusCode::OK => match response.json::<CodeiumResponseOk>().await {
                            Ok(resp_ok) => {
                                if let Some(ref completion_items) = resp_ok.completion_items {
                                    let mut items = Vec::with_capacity(completion_items.len());
                                    let range = Range {
                                        start: Position {
                                            line: request.position_line,
                                            character: 0,
                                        },
                                        end: Position {
                                            line: request.position_line + 1,
                                            character: 0,
                                        },
                                    };
                                    for (idx, item) in completion_items.iter().enumerate() {
                                        if idx == request.suggestions {
                                            break;
                                        }
                                        let mut new_text = item.completion.text.to_owned();
                                        new_text.push('\n');
                                        items.push(CompletionItem {
                                            label: item.completion.text.trim().to_owned(),
                                            kind: Some(CompletionItemKind::TEXT),
                                            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                                                range,
                                                new_text,
                                            })),
                                            ..Default::default()
                                        });
                                    }
                                    items
                                } else {
                                    vec![]
                                }
                            }
                            Err(error) => {
                                warn!("JsonOk Error: {:?}", error);
                                vec![CompletionItem::new_simple(
                                    format!("{:?}", error),
                                    "JsonOk Err".to_owned(),
                                )]
                            }
                        },
                        StatusCode::BAD_REQUEST => {
                            match response.json::<CodeiumResponseErr>().await {
                                Ok(resp_err) => {
                                    info!("ResponseErr Value: {:?}", resp_err);
                                    vec![CompletionItem::new_simple(
                                        resp_err.code,
                                        resp_err.message,
                                    )]
                                }
                                Err(error) => {
                                    warn!("JsonErr Error: {:?}", error);
                                    vec![CompletionItem::new_simple(
                                        format!("{:?}", error),
                                        "JsonErr Err".to_owned(),
                                    )]
                                }
                            }
                        }
                        _ => {
                            info!("http error: {}", status);
                            vec![CompletionItem::new_simple(
                                "".to_owned(),
                                "Json Ok Error".to_owned(),
                            )]
                        }
                    };
                    Ok(Some(CompletionResponse::Array(completion_response)))
                }
                Err(error) => {
                    info!("response error: {}", error);
                    Ok(None)
                }
            }
        })
    }
}

#[derive(Serialize)]
struct CodeiumRequest {
    metadata: Metadata,
    document: Document,
    editor_options: EditorOptions,
    other_documents: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    ide_name: String,          //"web"
    ide_version: String,       // "unknown"
    extension_version: String, // "1.6.13"
    extension_name: String,    // "helix-gpt"
    api_key: String,
    session_id: String,
}

#[derive(Serialize)]
struct Document {
    editor_language: String,
    language: usize,
    cursor_offset: usize,
    line_ending: String, // "\n"
    absolute_path: String,
    relative_path: String,
    text: String,
}

#[derive(Serialize)]
struct EditorOptions {
    tab_size: usize, //2
    insert_spaces: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CodeiumResponseOk {
    completion_items: Option<Vec<CodeiumCompletionItems>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CodeiumCompletionItems {
    completion: CodeiumCompletion,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CodeiumCompletion {
    text: String,
}

#[derive(Deserialize, Debug)]
struct CodeiumResponseErr {
    code: String,
    message: String,
}
