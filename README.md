# llm-lsp

Language Server Protocol for Large Language Models

Editors will be able to use this as lsp for code completions.

#### Current Features:

    [x] Code completion using codeium.ai
    [x] Uses async
    [x] Allows users to use same binary for different AI models with cli option
    [x] Saves multiple configs to be used by different instances of llm-lsp


#### How to install:

    cargo install llm-lsp

#### How to build locally:

    cargo build --release

#### Usage: 

##### Display help

    llm-lsp -h

##### Display version

    llm-lsp -V

##### Generate Config
Once llm-lsp is executed, it will guide through the process to generate API_KEY (in case of codeium) and save the relevant configurations in OS specific toml configs.

    llm-lsp generate-config

##### Configure editor command

    llm-lsp server -p codeium


##### Helix Editor configuration

    [language-server.llm-lsp]
    command = "llm-lsp"
    args = ["server", "-p", "codeium"]

    [[language]]
    name = "rust"
    language-servers = [
        "rust-analyzer",
        "llm-lsp",
    ]




#### Future:

    [ ] More Documentation
    [ ] Add chat support on cli
