mod configs;
mod providers;
mod server;
mod state;

use clap::{Parser, Subcommand};
use server::run_lsp;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::SetProvider { provider } => {
                // set provider config
            }
            Commands::GenerateAuth { provider } => {
                // generate auth token in config
            }
        },
        None => {
            // run lsp-llm server
            run_lsp().await;
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
    /// Change provider in .config/llm-lsp/config.toml
    #[command(arg_required_else_help = true)]
    SetProvider {
        /// Name of the provider config
        #[arg(short, long)]
        provider: String,
    },
    /// Generate auth token & save config in .config/llm-lsp/config.toml
    #[command(arg_required_else_help = true)]
    GenerateAuth {
        /// Name of the person to greet
        #[arg(short, long)]
        provider: String,
    },
}
