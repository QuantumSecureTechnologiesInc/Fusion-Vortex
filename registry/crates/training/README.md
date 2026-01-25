# Fusion Training

**Version:** 0.2.0
**Type:** ML Logic
**License:** MIT

## Overview

Fusion Training (`fusion_training`) provides the fundamental building blocks for machine learning training loops in Fusion. It includes standard implementations of epochs, batching, and evaluation metrics.

## Features

- **Training Loop**: Boilerplate-free training loop abstraction
- **Checkpoints**: Automatic saving and resuming of model state
- **Callbacks**: Hook system for logging and scheduling
- **Metrics**: Standard accuracy, loss, and F1 calculations

## Usage

```rust
use fusion_training::{Trainer, TrainerConfig};

let config = TrainerConfig::default().epochs(10);
let trainer = Trainer::new(model, optimizer, config);

trainer.fit(train_loader, val_loader).await?;
```text

## Dependencies

- `fusion_core`
- `fusion_ai_core`
- `rand`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)