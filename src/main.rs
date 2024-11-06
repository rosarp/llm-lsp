mod configs;
mod providers;
mod server;
mod state;

use clap::{Parser, Subcommand};
use inquire::{error::InquireError, Select};
use providers::codeium;
use server::run_lsp;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => {
            match command {
                Commands::Server { provider } => {
                    // get Api
                    match provider.as_str() {
                        "codeium" => {}
                        "ollama" | "openai" | "copilot" => {
                            println!("{provider} is not supported yet");
                            return;
                        }
                        _ => {
                            println!("Invalid provider: {provider}");
                            return;
                        }
                    }
                    // run lsp-llm server
                    // pass Api
                    run_lsp(provider).await;
                }
                Commands::GenerateConfig => {
                    let providers: Vec<&str> = vec!["codeium"];
                    let selected_provider: Result<&str, InquireError> =
                        Select::new("Please select provider to generate config.", providers)
                            .prompt();

                    match selected_provider {
                        Ok(provider) => match provider {
                            "codeium" => codeium::Codeium::generate_api_key().await,
                            "ollama" | "openai" | "copilot" => println!("{provider} is not supported yet"),
                            _ => println!("Please specify a valid provider. To check valid providers run `llm-lsp list-providers`"),
                        },
                        Err(error) =>println!("There was an error, please try again: {error}"),
                    }
                }
                Commands::ListProviders => {
                    println!(
                        r#"
                        Following providers are supported as of now:
                            1.codeium
                        "#
                    );
                    println!("In future more providers will be supported such as ollama, openai, copilot.");
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
    /// Run `llm-lsp generate-auth` command before running this command
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
    /// List supported providers
    ListProviders,
}
