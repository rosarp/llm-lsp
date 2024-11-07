use crate::configs::LlmConfig;
use inquire::Editor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    name: String,
    api_key: String,
}

pub async fn generate_api_key() {
    let session_id = Uuid::new_v4().to_string();
    let auth_url = format!("https://www.codeium.com/profile?response_type=token&redirect_uri=vim-show-auth-token&state={}&scope=openid%20profile%20email&redirect_parameters_type=query", session_id);

    let token = Editor::new("Authentication Token:")
        .with_help_message(format!("Visit the following URL: {}", auth_url).as_str())
        .prompt();

    match token {
        Ok(auth_token) => {
            let auth_token = auth_token.trim().to_owned();
            match register(auth_token).await {
                Ok(api_key) => {
                    let mut config_map = HashMap::new();
                    config_map.insert("API_KEY".to_owned(), api_key);
                    config_map.insert("SESSION_ID".to_owned(), session_id);
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
        .map_err(|error| format!("Post Error: {error}"))?
        .json::<RegisterResponse>()
        .await
        .map_err(|error| format!("Json Error: {error}"))?
        .api_key;
    Ok(api_key)
}
