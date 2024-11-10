use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct LspConfig<'a> {
    pub commands: Vec<Command<'a>>,
    pub trigger_characters: Vec<&'a str>,
}

impl<'a> LspConfig<'a> {
    pub fn init() -> Self {
        LspConfig {
            commands: Self::get_commands(),
            trigger_characters: Self::get_trigger_characters(),
        }
    }

    fn get_commands() -> Vec<Command<'a>> {
        vec![
            Command {
                key: "resolve_diagnostics",
                label: "Resolve diagnostics",
                query: "Resolve the diagnostics for this code.",
            },
            Command {
                key: "generate_docs",
                label: "Generate documentation",
                query: "Add documentation to this code.",
            },
            Command {
                key: "improve_code",
                label: "Improve code",
                query: "Improve this code.",
            },
            Command {
                key: "refactor_from_comment",
                label: "Refactor code from a comment",
                query: "Refactor this code based on the comment.",
            },
            Command {
                key: "write_test",
                label: "Write a unit test",
                query: "Write a unit test for this code. Do not include any imports.",
            },
        ]
    }

    fn get_trigger_characters() -> Vec<&'a str> {
        vec!["{", "(", " "]
    }
}

pub struct Command<'a> {
    pub key: &'a str,
    #[allow(unused)]
    label: &'a str,
    #[allow(unused)]
    query: &'a str,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LlmConfig {
    codeium: HashMap<String, String>,
    ollama: HashMap<String, String>,
    openapi: HashMap<String, String>,
    copilot: HashMap<String, String>,
}

impl LlmConfig {
    pub fn generate_config(
        provider: String,
        config_map: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut llm_config: LlmConfig = confy::load("llm-lsp", None).unwrap();
        match provider.as_str() {
            "codeium" => llm_config.codeium.extend(config_map),
            _ => return Err("Provider {provider} is not supported as of now!".to_owned()),
        };
        confy::store("llm-lsp", None, llm_config).unwrap();
        Ok(())
    }

    pub fn get_configs(provider: &str) -> Result<HashMap<String, String>, String> {
        let llm_config: LlmConfig = confy::load("llm-lsp", None).unwrap();
        match provider {
            "codeium" => Ok(llm_config.codeium),
            _ => Err("Provider {provider} is not supported as of now!".to_owned()),
        }
    }
}
