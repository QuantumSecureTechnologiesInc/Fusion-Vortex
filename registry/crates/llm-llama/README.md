# Fusion LLM Llama

**Version:** 0.2.0
**Type:** Model Implementation
**License:** MIT

## Overview

Fusion LLM Llama (`fusion_llm_llama`) is a highly optimized Rust implementation of the LLaMA model architecture (Llama 1, 2, and 3). It supports advanced features like RoPE (Rotary Positional Embeddings) and RMSNorm.

## Features

- **Architecture**: Complete implementation of LLaMA transformer blocks
- **Compatibility**: Supports loading weights from GGUF, Safetensors, and PyTorch
- **Optimization**: Uses fused kernels for critical ops

## Usage

```rust
use fusion_llm_llama::LlamaModel;

let model = LlamaModel::from_file("llama-3-8b.safetensors")?;
let logits = model.forward(input_ids)?;
```text

## Dependencies

- `fusion_ai_core`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)