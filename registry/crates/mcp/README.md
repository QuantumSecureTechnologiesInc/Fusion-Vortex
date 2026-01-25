# Fusion MCP (Model Context Protocol)

**Version:** Workspace
**Type:** AI Assistant Integration
**License:** MIT / Apache 2.0 Dual License

## Overview

Fusion MCP provides integration with the Model Context Protocol, enabling AI assistants to deeply understand and interact with Fusion codebases. It bridges AI models with the Fusion runtime, providing context-aware code assistance and intelligent suggestions.

## Features

- **Deep Code Understanding**: Provides AI models with full AST, type information, and semantic context
- **Runtime Integration**: Built on `fusion_runtime_core` for async operations
- **Policy Enforcement**: Integrates with `fusion-policy` for secure AI interactions
- **VSCode Integration**: Works seamlessly with Fusion's VSCode extension
- **Real-time Updates**: Maintains synchronized state with code changes

## Architecture

```text
AI Assistant (Claude, GPT-4, etc.)
    ↓ MCP Protocol
Fusion MCP Server
    ↓
┌─────────────────────────────────┐
│  fusion-vscode-runtime          │
│  (Monolith Integration)         │
└─────────────────────────────────┘
    ↓
fusion_runtime_core
    ↓
Fusion Project (AST, Types, Symbols)
```text

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fusion-mcp = { workspace = true }
```text

## Usage

### As a Library

```rust
use fusion_mcp::{McpServer, McpConfig};
use fusion_runtime_core::Runtime;

#[tokio::main]

async fn main() -> Result<(), anyhow::Error> {
    let runtime = Runtime::new();
    let config = McpConfig::default();

    let server = McpServer::new(config, runtime);
    server.start().await?;

    Ok(())
}
```text

### MCP Protocol Endpoints

The server exposes these MCP protocol methods:

- `mcp/getContext`: Retrieve code context at cursor position
- `mcp/suggest`: Get AI-powered code suggestions
- `mcp/analyze`: Perform deep semantic analysis
- `mcp/refactor`: Get refactoring suggestions

### Integration with AI Assistants

```rust
use fusion_mcp::client::McpClient;

let client = McpClient::connect("http://localhost:3030").await?;

// Get context for AI
let context = client.get_context("src/main.rs", 42).await?;

// Request suggestions
let suggestions = client.suggest(context).await?;
```text

## Configuration

Environment variables:

- `FUSION_MCP_PORT`: Server port (default: `3030`)
- `FUSION_MCP_HOST`: Server host (default: `127.0.0.1`)
- `FUSION_MCP_LOG_LEVEL`: Logging verbosity (default: `info`)

## Security

- All AI interactions are policy-governed via `fusion-policy`
- Sandboxed execution prevents unauthorized code modification
- Request validation ensures safe AI suggestions
- Audit logging tracks all AI interactions

## Dependencies

- `fusion_runtime_core`: Async runtime
- `fusion-vscode-runtime`: VSCode/Monolith integration
- `fusion-policy`: Security policy enforcement
- `serde`/`serde_json`: Serialization
- `reqwest`: HTTP client
- `uuid`: Request tracking

## See Also

- [VSCode Runtime](../../../crates/vscode-runtime)
- [Policy Engine](../../../crates/fusion-policy)
- [Fusion Runtime Core](../fusion_runtime_core)

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)

## License

MIT OR Apache-2.0