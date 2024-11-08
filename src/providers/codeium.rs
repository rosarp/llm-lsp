use super::{
    codeium_types,
    llm_api::{CompletionRequest, LlmClientApi, LlmState},
};
use async_lsp::{
    lsp_types::{CompletionItem, CompletionResponse},
    ResponseError,
};
use futures::future::BoxFuture;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT_ENCODING, AUTHORIZATION, CONNECTION, CONTENT_TYPE,
};
use serde::Serialize;
use tracing::info;

impl LlmClientApi for LlmState {
    fn new(api_key: &str, session_id: &str) -> LlmState {
        let auth_url = "https://web-backend.codeium.com/exa.language_server_pb.LanguageServerService/GetCompletions".to_owned();
        let mut headers = HeaderMap::with_capacity(2);
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
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let client = reqwest::Client::builder()
            .default_headers(headers.clone())
            .build()
            .unwrap();
        LlmState {
            auth_url,
            headers,
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
        let text = request.contents;
        let lines = text.split("\n").collect::<Vec<&str>>();
        for i in 0..request.position_line {
            if let Some(&ln) = lines.get(i as usize) {
                cursor_offset += ln.len();
            }
        }
        // The editor name needs to be known by codeium
        // The extensionVersion needs to a recent one, so codeium accepts it
        let request_body = CodeiumRequest {
            metadata: Metadata {
                ide_name: "web".to_owned(),
                ide_version: "unknown".to_owned(),
                extension_version: "1.6.13".to_owned(),
                extension_name: "helix-gpt".to_owned(),
                api_key: self.api_key.clone(),
                session_id: self.session_id.clone(),
            },
            document: Document {
                editor_language: request.language_id,
                language,
                cursor_offset: cursor_offset + request.position_char as usize,
                line_ending: "\n".to_owned(),
                absolute_path: request.filepath.clone(),
                relative_path: request.filepath.clone(),
                text,
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
                    let llm_response = response.text().await.unwrap();
                    info!("response: {}", llm_response);
                    let completion_response = vec![CompletionItem::new_simple(
                        llm_response,
                        "description".to_owned(),
                    )];
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
struct Metadata {
    #[serde(rename = "ideName")]
    ide_name: String, //"web"
    #[serde(rename = "ideVersion")]
    ide_version: String, // "unknown"
    #[serde(rename = "extensionVersion")]
    extension_version: String, // "1.6.13"
    #[serde(rename = "extensionName")]
    extension_name: String, // "helix-gpt"
    #[serde(rename = "apiKey")]
    api_key: String,
    #[serde(rename = "sessionId")]
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
