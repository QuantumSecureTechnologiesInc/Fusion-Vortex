# Fusion LLM Distributed Training

**Version:** 0.2.0
**Type:** Training System
**License:** MIT

## Overview

Fusion LLM Distributed Training (`fusion_llm_distributed_training`) enables scaling LLM training across multiple GPUs and nodes. It implements standard parallelism strategies like FSDP (Fully Sharded Data Parallel) and Tensor Parallelism.

## Features

- **Data Parallel**: Replicates model, splits data
- **Tensor Parallel**: Splits individual layers across GPUs (Megatron-style)
- **Pipeline Parallel**: Splits layers across nodes
- **ZeRO Optimization**: Shards optimizer states and gradients

## Usage

```rust
use fusion_llm_distributed_training::Trainer;

// Initialize distributed process group
let rank = std::env::var("RANK")?.parse()?;
let world_size = std::env::var("WORLD_SIZE")?.parse()?;

let trainer = Trainer::new(model, optimizer, rank, world_size);
trainer.train(dataset).await?;
```text

## Dependencies

- `fusion_core`
- `fusion_net`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)