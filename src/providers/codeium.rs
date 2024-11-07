use super::{
    codeium_types,
    llm_api::{CompletionRequest, LlmClientApi, LlmState},
};
use async_lsp::{
    lsp_types::{CompletionItem, CompletionResponse},
    ResponseError,
};
use futures::future::BoxFuture;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use tracing::info;

impl LlmClientApi for LlmState {
    fn new(api_key: &str, session_id: &str) -> LlmState {
        let auth_url = "https://web-backend.codeium.com/exa.language_server_pb.LanguageServerService/GetCompletions".to_owned();
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            format!("Basic {}-{}", api_key, session_id)
                .as_str()
                .parse()
                .unwrap(),
        );
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
            .unwrap_or(&0usize);
        // The editor name needs to be known by codeium
        // The extensionVersion needs to a recent one, so codeium accepts it
        let request_body = format!(
            r#"{{
  "metadata": {{
    "ideName": "web",
    "ideVersion": "unknown",
    "extensionVersion": "1.6.13",
    "extensionName": "helix-gpt",
    "apiKey": {},
    "sessionId": {}
  }},
  "document": {{
    "editor_language": {},
    "language": {},
    "cursor_offset": {},
    "line_ending": "\n",
    "absolute_path": {},
    "relative_path": {},
    "text": {}
  }},
  "editor_options": {{
    "tab_size": 2,
    "insert_spaces": true
  }},
  "other_documents": []
}}"#,
            self.api_key,
            self.session_id,
            request.language_id,
            language,
            request.position_char,
            request.filepath,
            request.filepath,
            request.contents
        );
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
