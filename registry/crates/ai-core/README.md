# Fusion AI Core

**Version:** Workspace
**Type:** Core Library
**License:** MIT / Apache 2.0 Dual License

## Overview

Fusion AI Core (`fusion-ai-core`) is the foundational library for all AI capabilities in the Fusion ecosystem. It provides unified interfaces for model inference, safety policies, prompt engineering, and context management.

## Key Features

- **Model Agnostic Interfaces**: Unified API for OpenAI, Anthropic, Local LLMs, and custom models
- **Safety & Policy Enforcement**: Pre-computation checks and post-generation validation
- **Context Management**: Efficient handling of token windows and context truncation
- **Prompt Templates**: Type-safe templates for structured interactions
- **Secure Integration**: Interactions verified against `fusion-policy`

## Usage

```rust
use fusion_ai_core::{AiClient, ModelConfig, Prompt};

async fn generate_text() -> Result<String, anyhow::Error> {
    let client = AiClient::new(ModelConfig::default());

    let prompt = Prompt::new("Explain quantum entanglement")
        .with_system("You are a physics expert");

    let response = client.complete(prompt).await?;
    Ok(response)
}
```text

## Architecture

- **Adapters**: Connectors for different model providers
- **Policies**: Safety rules and content filtering
- **Tokenizers**: Utilities for token counting and management
- **Embeddings**: Vector generation for semantic search

## Security

Includes built-in support for Post-Quantum Cryptography (via `pqcrypto-mldsa`) to sign and verify model outputs, ensuring provenance and integrity.

## Dependencies

- `fusion_runtime_core`
- `fusion_core`
- `pqcrypto-mldsa`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)