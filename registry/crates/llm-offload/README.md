# Fusion LLM Offload

**Version:** 0.2.0
**Type:** Resource Management
**License:** MIT

## Overview

Fusion LLM Offload (`llm-offload`) manages the intelligent offloading of model weights and KV cache layers to system RAM (CPU) or NVMe SSDs when VRAM is exhausted.

## Features

- **Smart Prefetching**: Predicts required layers and prefetches them to GPU
- **Pinned Memory**: Uses pinned CPU memory for faster transfers
- **Asynchronous Copy**: Overlaps compute with data transfer

## Usage

```rust
use llm_offload::{Offloader, Strategy};

let offloader = Offloader::new(Strategy::CpuRam);
// Move layer to CPU
offloader.offload(layer_tensor).await?;
// Bring back to GPU
let gpu_tensor = offloader.load(layer_tensor).await?;
```text

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)