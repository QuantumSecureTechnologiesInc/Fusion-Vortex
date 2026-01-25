# Fusion LLM Inference Engine

**Version:** 0.2.0
**Type:** Core Engine
**License:** MIT

## Overview

Fusion LLM Inference Engine (`fusion_llm_inference_engine`) is the high-performance runtime for executing Large Language Models. It abstracts over specific model architectures to provide a unified generation API.

## Features

- **Unified API**: Run Llama, Mistral, Falcon, etc. through a single interface
- **Optimized**: Integrates with `fusion_llm_gpu_scheduler` for VRAM efficiency
- **Sampling**: Advanced sampling strategies (Top-P, Top-K, Min-P, Mirostat)

## Usage

```rust
use fusion_llm_inference_engine::{Engine, Request};

let engine = Engine::load("llama-3-8b")?;
let stream = engine.generate("Hello, world!").await?;

while let Some(token) = stream.next().await {
    print!("{}", token);
}
```text

## Dependencies

- `fusion_core`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)