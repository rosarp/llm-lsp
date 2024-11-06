use crate::{configs::LlmConfig, providers::llm_api::BaseApi};
use inquire::Editor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Codeium {
    auth_url: String,
    session_id: String,
    api_key: String,
}

impl Codeium {
    pub async fn generate_api_key() {
        let auth_url = "https://www.codeium.com/profile?response_type=token&redirect_uri=vim-show-auth-token&state=a&scope=openid%20profile%20email&redirect_parameters_type=query".to_owned();

        let token = Editor::new("Authentication Token:")
            .with_help_message(format!("Visit the following URL: {}", auth_url).as_str())
            .prompt();

        match token {
            Ok(auth_token) => {
                let auth_token = auth_token.trim().to_owned();
                match Codeium::register(auth_token).await {
                    Ok(api_key) => {
                        let mut config_map = HashMap::new();
                        config_map.insert("API_KEY".to_owned(), api_key);
                        _ = LlmConfig::generate_config("codeium".to_owned(), config_map);
                    }
                    Err(error) => println!("Error registering: {}", error),
                }
            }
            Err(error) => println!("Input Error: {error}"),
        }
    }

    async fn register(auth_token: String) -> Result<String, String> {
        let mut payload = HashMap::new();
        payload.insert("firebase_id_token", auth_token.as_str());
        let req_client = reqwest::Client::new();
        let api_key = req_client
            .post("https://api.codeium.com/register_user/")
            .json(&payload)
            .send()
            .await
            .map_err(|error| format!("Error: {error}"))?
            .json::<CodeiumRegisterResponse>()
            .await
            .map_err(|error| format!("Error: {error}"))?
            .api_key;
        Ok(api_key)
    }
}

#[derive(Serialize, Deserialize)]
struct CodeiumRegisterResponse {
    name: String,
    api_key: String,
}
