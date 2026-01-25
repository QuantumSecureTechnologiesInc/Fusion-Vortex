# Fusion LLM Cache Compression

**Version:** 0.2.0
**Type:** Optimization
**License:** MIT

## Overview

Fusion LLM Cache Compression (`fusion_llm_cache_compression`) implements advanced algorithms for compressing the Key-Value (KV) cache during LLM inference. This significantly reduces VRAM usage, allowing for longer context windows or larger batch sizes.

## Features

- **Quantization**: Supports 8-bit and 4-bit KV cache quantization
- **Sparsity**: Evicts less important tokens based on attention scores
- **Paged Attention**: Integrates with memory paging systems

## Usage

```rust
use fusion_llm_cache_compression::{Compressor, Method};

let compressor = Compressor::new(Method::Int8);
let compressed_kv = compressor.compress(&kv_cache)?;
```text

## Dependencies

- `fusion_core`
- `fusion_llm_quantization`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)