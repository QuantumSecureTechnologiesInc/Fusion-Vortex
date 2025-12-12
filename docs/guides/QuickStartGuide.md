# Fusion VSC CLI Quick Start Guide

Welcome to the Fusion VSC CLI! This guide will get you up and running with the bridge between your AI models and VS Code extensions.

## Prerequisites

*   **Rust Toolchain**: 1.75+
*   **Node.js**: (Optional) For running external extension processors if needed.
*   **VS Code**: Installed (for extension discovery).

## Installation

```bash
git clone https://github.com/fusion-lang/fusion-vsc-cli.git
cd fusion-vsc-cli
cargo build --release -p fusion
```

## Core Workflows

### 1. Starting the MCP Server

The MCP server acts as the gateway.

```bash
# Start server on port 3000
./target/release/fusion mcp serve --port 3000
```

### 2. Validating the Bridge

Check if the CLI can see your extensions:

```bash
./target/release/fusion extensions list
```

### 3. Using AI Assistance

To ask the AI to perform a task using available tools:

```bash
./target/release/fusion ai assist "Refactor the current file using the VS Code formatter"
```

## Troubleshooting

*   **"Extension Host not initialized"**: Ensure you have run `fusion mcp serve`.
*   **"Command not found"**: Check `fusion extensions list` to see registered capabilities.
