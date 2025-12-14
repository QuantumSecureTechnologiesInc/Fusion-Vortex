# Fusion Tensor Parallel

**Version:** 0.2.0  
**Type:** Distributed ML  
**License:** MIT

## Overview

Fusion Tensor Parallel (`fusion_llm_tensor_parallel`) implements model parallelism by splitting individual tensor operations across multiple GPUs. This follows the Megatron-LM style of splitting Matrix Multiplications (Row/Column Parallel).

## Features

- **Column Parallel Linear**: Splits weight matrix by columns
- **Row Parallel Linear**: Splits weight matrix by rows
- **All-Reduce**: Efficient synchronization of partial results
- **Vocab Parallel**: Splits large vocabulary embeddings

## Usage

```rust
use fusion_llm_tensor_parallel::{ColumnParallelLinear, ParallelContext};

let ctx = ParallelContext::from_env()?;
let layer = ColumnParallelLinear::new(input_dim, output_dim, &ctx)?;

// Forward pass executes locally, then synchronizes if needed
let output = layer.forward(input)?;
```

## Dependencies

- `fusion_core`
- `fusion_net`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
