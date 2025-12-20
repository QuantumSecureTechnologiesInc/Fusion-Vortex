# Fusion LLM Quantization

**Version:** 0.2.0  
**Type:** Optimization  
**License:** MIT

## Overview

Fusion LLM Quantization (`fusion_llm_quantization`) provides primitives for reducing the precision of LLM weights and activations. It supports standard integer quantization schemes to reduce memory footprint and increase inference throughput.

## Features

- **Weight Only**: INT8 and INT4 weight quantization (GPTQ style)
- **Activation**: Dynamic activation quantization
- **KV Cache**: Quantized Key-Value cache storage
- **Kernels**: Optimized dequantization kernels for typical operations

## Usage

```rust
use fusion_llm_quantization::{Quantizer, Method};

let quantizer = Quantizer::new(Method::Int4Sym);
let quantized_tensor = quantizer.quantize(fp16_tensor)?;
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
