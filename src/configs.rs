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
    label: &'a str,
    query: &'a str,
}
