# Fusion v2.0 Vortex VSC CLI Quick Start Guide

Welcome to the Fusion v2.0 Vortex VSC CLI! This guide will get you up and running with the bridge between your AI models and VS Code extensions.

## Prerequisites

* **Fusion Toolchain**: Use `./install.sh` to provision `dist/`
* **Node.js**: (Optional) For running external extension processors if needed.
* **VS Code**: Installed (for extension discovery).

## Installation

```bash
git clone https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language
cd "Fusion - Programming Language"
./install.sh

export PATH="$PATH:$(pwd)/dist/bin"
```text

## Core Workflows

### 1. Starting the MCP Server

The MCP server acts as the gateway.

```bash

# Start server on port 3000

fusion mcp serve --port 3000
```text

### 2. Validating the Bridge

Check if the CLI can see your extensions:

```bash
fusion extensions list
```text

### 3. Using AI Assistance

To ask the AI to perform a task using available tools:

```bash
fusion ai assist "Refactor the current file using the VS Code formatter"
```text

## Troubleshooting

* **"Extension Host not initialized"**: Ensure you have run `fusion mcp serve`.
* **"Command not found"**: Check `fusion extensions list` to see registered capabilities.
