# Fusion LLM Dynamic Batch

**Version:** 0.2.0
**Type:** Inference Optimization
**License:** MIT

## Overview

Fusion LLM Dynamic Batch (`llm-dynamic-batch`) implements continuous batching (also known as cellular batching) for high-throughput LLM serving. It allows requests to join and leave a running batch iteration dynamically.

## Features

- **Continuous Batching**: No waiting for padding; requests are packed tightly
- **Preemption**: Low-priority requests can be paused for high-priority ones
- **Fairness**: Ensures fairness in token generation speeds across users

## Usage

```rust
use llm_dynamic_batch::BatchScheduler;

let scheduler = BatchScheduler::new(model);

// Add requests asynchronously
scheduler.submit("Request 1").await;
scheduler.submit("Request 2").await;

// Scheduler automatically forms optimal batches
scheduler.run_loop().await?;
```text

## Dependencies

- `fusion_core`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)