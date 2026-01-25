# Fusion LLM LoRA Kernel

**Version:** 0.2.0
**Type:** Compute Kernel
**License:** MIT

## Overview

Fusion LLM LoRA Kernel (`fusion_llm_lora_kernel`) implements the mathematical operations required for Low-Rank Adaptation (LoRA). It efficiently computes $W + AB$ without materializing the full weight matrix, saving memory.

## Features

- **Zero-Copy**: Applies adapters on the fly
- **Multi-Adapter**: Supports stacking multiple LoRAs
- **Fused Op**: Custom CUDA/Metal kernel for `y = x(W + AB)`

## Usage

```rust
use fusion_llm_lora_kernel::apply_lora;

// y = x * W + x * A * B * scaling
let y = apply_lora(x, w, lora_a, lora_b, scaling)?;
```text

## Dependencies

- `fusion_ai_core`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)