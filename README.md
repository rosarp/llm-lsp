# llm-lsp

A Language Server Protocol (LSP) implementation for integrating Large Language Models into text editors. This project enables developers to leverage AI-powered code completions directly within their favorite editors that support the LSP protocol.

## Overview

llm-lsp serves as a bridge between your text editor and AI code completion services. Currently supporting Codeium.ai, it provides intelligent code suggestions while you type, enhancing your coding productivity without leaving your editor.

## Features

- **AI-Powered Code Completion**
  - Integration with [Codeium.ai](https://codeium.ai) for intelligent code suggestions
  - Real-time completion as you type
  - Context-aware suggestions based on your codebase

- **Performance**
  - Asynchronous operation for responsive editing experience
  - Efficient communication between editor and AI service

- **Flexibility**
  - Support for multiple AI models through CLI options
  - Multiple configuration profiles for different use cases
  - Easy configuration management with OS-specific TOML configs

- **Editor Support**
  - Seamless integration with LSP-compatible editors
  - Detailed setup instructions for Helix editor
  - Extensible design for future editor support

## Installation

### From Cargo (Recommended)

```bash
cargo install llm-lsp
```

### Build from Source

1. Clone the repository
2. Build the project:
```bash
cargo build --release
```
3. The binary will be available in `target/release/llm-lsp`

## Usage

### Initial Setup

1. Generate configuration:
```bash
llm-lsp generate-config
```
This will guide you through setting up your API key and other configurations.

### Command Line Interface

- Show help:
```bash
llm-lsp -h
```

- Display version:
```bash
llm-lsp -V
```

- Start LSP server with Codeium provider:
```bash
llm-lsp server -p codeium
```

### Editor Configuration

#### Helix Editor

Add the following to your Helix configuration:

```toml
[language-server.llm-lsp]
command = "llm-lsp"
args = ["server", "-p", "codeium"]

[[language]]
name = "rust"  # Or any supported language
language-servers = [
    "rust-analyzer",  # Or any supported language server
    "llm-lsp",
]
```

## Configuration

The configuration file is automatically created in the OS-specific config directory:
- Linux: `~/.config/llm-lsp`
- macOS: `~/Library/Application Support/llm-lsp`
- Windows: `%APPDATA%\llm-lsp`

## Roadmap

- [ ] CLI-based chat support
- [ ] Support for additional AI providers
- [ ] More editor-specific configurations
- [ ] Improved completion context handling

## Contributing

Contributions are welcome! However, please open an issue to discuss larger changes, to avoid doing a lot of work that may get rejected.

## License

This project is licensed under either of:
- MIT license
- Apache License, Version 2.0

at your option.

---
For more information, bug reports, or feature requests, please visit the [GitHub repository](https://github.com/rosarp/llm-lsp).
