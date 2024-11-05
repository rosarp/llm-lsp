use std::collections::HashMap;

pub mod codeium;
pub mod llm_api;
pub mod ollama;

pub struct AiModelOptions {
    url: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
}

trait AiModel {
    fn auth();
    fn completion();
    fn chat();
}
