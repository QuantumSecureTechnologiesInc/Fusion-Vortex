# Fusion Tensor Optim

**Version:** 0.2.0
**Type:** ML Training
**License:** MIT

## Overview

Fusion Tensor Optim (`fusion_tensor_optim`) provides advanced optimization utilities for training neural networks. It implements gradient handling strategies beyond simple updates.

## Features

- **Gradient Accumulation**: Simulates larger batch sizes
- **Gradient Clipping**: Prevents exploding gradients (Norm and Value clipping)
- **Weight Decay**: Regularization application
- **EMA**: Exponential Moving Average of weights

## Usage

```rust
use fusion_tensor_optim::OptimizerUtils;

// Clip gradients to max norm of 1.0
let total_norm = gradients.clip_norm(1.0)?;

// Apply accumulated gradients
optimizer.step(&accumulated_grads)?;
```text

## Dependencies

- `fusion_core`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)