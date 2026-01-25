# Fusion LLM GQA Kernel

**Version:** 0.2.0
**Type:** Evaluation Kernel
**License:** MIT

## Overview

Fusion LLM GQA Kernel (`fusion_llm_gqa_kernel`) provides highly optimized, fused CUDA and Metal implementations of Grouped Query Attention (GQA). This is critical for the performance of modern models like Llama-2-70B and Mistral.

## Features

- **Fused Ops**: Combines query grouping, scaling, and attention logic into single kernels
- **Flash Attention**: Integrates Flash Attention principles for memory efficiency
- **Mixed Precision**: Optimized for fp16 and bf16 execution

## Usage

```rust
use fusion_llm_gqa_kernel::gqa_forward;

let output = gqa_forward(
    query, key, value,
    num_query_heads, num_kv_heads
)?;
```text

## Dependencies

- `fusion_ai_core`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)