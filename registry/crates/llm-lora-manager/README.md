# Fusion LLM LoRA Manager

**Version:** 0.2.0  
**Type:** Model Management  
**License:** MIT

## Overview

Fusion LLM LoRA Manager (`fusion_llm_lora_manager`) orchestrates the loading, caching, and hot-swapping of LoRA (Low-Rank Adaptation) adapters. It allows a single base model to serve many different fine-tuned tasks simultaneously.

## Features

- **Multi-Tenant**: Serve hundreds of adapters on one base model
- **Hot-Swapping**: Switch adapters per-request with zero downtime
- **LRU Caching**: Managed VRAM cache for frequently used adapters
- **Merging**: Utilities to permanently merge adapters into base weights

## Usage

```rust
use fusion_llm_lora_manager::{LoraManager, AdapterConfig};

let manager = LoraManager::new(model_config);
let adapter_id = manager.load_adapter("sql-tuner", "path/to/lora").await?;

// Use in inference
let output = model.generate(input, &mut manager.get(adapter_id).context());
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
