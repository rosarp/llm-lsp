use async_lsp::{
    lsp_types::{CompletionResponse, Position},
    ResponseError,
};
use futures::future::BoxFuture;
use reqwest::Client;
use ropey::Rope;

pub struct CompletionRequest {
    pub contents: Rope,
    pub filepath: String,
    pub language_id: String,
    pub position: Position,
    pub suggestions: usize,
    #[allow(unused)]
    pub client_name: String,
    #[allow(unused)]
    pub client_version: String,
}

pub struct LlmState {
    pub auth_url: String,
    pub api_key: String,
    pub session_id: String,
    pub client: Client,
}

pub trait LlmClientApi {
    fn new(api_key: &str, session_id: &str) -> Self;

    #[allow(unused)]
    fn chat(&self);

    fn completion(
        &self,
        completion_request: CompletionRequest,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, ResponseError>>;
}
