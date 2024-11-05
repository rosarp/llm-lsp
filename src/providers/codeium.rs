use crate::configs::LlmConfig;
use inquire::Editor;
use std::collections::HashMap;

pub struct Codeium {
    auth_url: String,
    session_id: String,
    api_key: String,
}

impl Codeium {
    fn auth() {}

    fn register(token: String) {}
}

pub fn generate_api_key() {
    let auth_url = "https://www.codeium.com/profile?response_type=token&redirect_uri=vim-show-auth-token&state=a&scope=openid%20profile%20email&redirect_parameters_type=query".to_owned();

    let uuid = Editor::new("Authentication Token:")
        .with_help_message(format!("Visit the following URL:\n {}", auth_url).as_str())
        .prompt();

    match uuid {
        Ok(api_key) => {
            let mut config_map = HashMap::new();
            config_map.insert("API_KEY".to_owned(), api_key.trim().to_owned());
            _ = LlmConfig::generate_config("codeium".to_owned(), config_map);
        }
        Err(error) => println!("Error: {error}"),
    }
}

fn register() {}
