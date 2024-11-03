use std::collections::HashMap;

mod ollama;
mod ollama_auth;

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
