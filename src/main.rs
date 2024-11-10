mod configs;
mod lsp;
mod providers;
mod server;
mod state;

use clap::{Parser, Subcommand};
use configs::LlmConfig;
use inquire::{error::InquireError, Select};
use providers::{
    codeium_auth,
    llm_api::{LlmClientApi, LlmState},
};
use server::LlmLanguageServer;
use tracing::warn;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => {
            match command {
                Commands::Server { provider } => {
                    let llm_config = match LlmConfig::get_configs(&provider) {
                        Ok(configs) => configs,
                        Err(_) => {
                            return;
                        }
                    };
                    let llm_client = match provider.as_str() {
                        "codeium" => {
                            let api_key = match llm_config.get("API_KEY") {
                                Some(k) => k,
                                None => {
                                    warn!("API_KEY not found in config");
                                    return;
                                }
                            };
                            let session_id = match llm_config.get("SESSION_ID") {
                                Some(s) => s,
                                None => {
                                    warn!("SESSION_ID not found in config");
                                    return;
                                }
                            };
                            LlmState::new(api_key, session_id)
                        }
                        "ollama" | "openai" | "copilot" => {
                            println!("{provider} is not supported yet");
                            return;
                        }
                        _ => {
                            println!("Invalid provider: {provider}");
                            return;
                        }
                    };
                    // run lsp-llm server
                    LlmLanguageServer::run(llm_client).await;
                }
                Commands::GenerateConfig => {
                    let providers: Vec<&str> = vec!["codeium"];
                    let selected_provider: Result<&str, InquireError> =
                        Select::new("Please select provider to generate config.", providers)
                            .prompt();

                    match selected_provider {
                        Ok(provider) => match provider {
                            "codeium" => codeium_auth::generate_api_key().await,
                            "ollama" | "openai" | "copilot" => println!("{provider} is not supported yet"),
                            _ => println!("Please specify a valid provider. To check valid providers run `llm-lsp list-providers`"),
                        },
                        Err(error) =>println!("There was an error, please try again: {error}"),
                    }
                }
            }
        }
        None => {
            println!("No valid command specified. Check for help with `llm-lsp -h`.");
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "llm-lsp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run `llm-lsp generate-config` command before running this command
    /// Run LSP server to connect with LLM
    #[command(arg_required_else_help = true)]
    Server {
        /// Name of the provider config
        #[arg(short, long)]
        provider: String,
    },
    /// Run this command before running `llm-lsp server` command
    /// Generate auth token & save config in .config/llm-lsp/default-config.toml
    GenerateConfig,
}
