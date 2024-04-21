pub fn get_commands<'a>() -> Vec<Command<'a>> {
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

pub fn get_trigger_characters<'a>() -> Vec<&'a str> {
    vec!["{", "(", " "]
}

pub struct Command<'a> {
    key: &'a str,
    label: &'a str,
    query: &'a str,
}
