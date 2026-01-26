# Advanced AI CLI

## Overview
The Fusion CLI (`fusion`) includes an embedded AI engine that rivals standalone coding assistants. It serves as a unified interface for coding, refactoring, testing, and system management, capable of running offline for maximum privacy.

## Unique Features

### 🔌 VS Code Runtime Integration
**World First**: The CLI includes a headless VS Code runtime (`vscode-runtime` crate).
- Run VS Code extensions **without** the editor.
- Use language servers, linters, and formatters directly from the terminal.

### 🤖 Multi-Provider Support
Switch AI backends instantly:
- **Cloud**: Claude 3.5 Sonnet, GPT-4 Turbo, Gemini Pro.
- **Local**: Llama 3, Mistral (runs completely offline).

### 🛠️ MCP Server
Full implementation of the **Model Context Protocol**:
- Exposes project context to external agents.
- Provides tools: `read_file`, `execute_command`, `git_commit` via secure API.

### 🛡️ Smart Capabilities
- **Security Review**: `fusion ai review --focus security`
- **Refactoring**: `fusion ai refactor --modernize`
- **Test Gen**: `fusion ai tests ./src/lib.fu`

## Interactive Mode
One command to rule them all:
```bash
$ fusion ai assist
> "Refactor the quantum module to use the new Supernova dispatcher"
[AI analyzes code, plans refactor, and applies changes...]
```
