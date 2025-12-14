# Fusion Tensor Sparse

**Version:** 0.2.0  
**Type:** Mathematics  
**License:** MIT

## Overview

Fusion Tensor Sparse (`fusion_tensor_sparse`) optimizes storage and computation for sparse tensors. It is useful for graph neural networks, large embedding tables, and scientific computing.

## Features

- **Formats**: COO (Coordinate), CSR (Compressed Sparse Row)
- **Operations**: Sparse-Dense MatMul, Sparse-Sparse addition
- **Acceleration**: Specialized kernels for CPU and GPU

## Usage

```rust
use fusion_tensor_sparse::SparseTensor;

let sparse = SparseTensor::from_coo(indices, values, shape)?;
let dense = sparse.to_dense()?;
```

## Dependencies

- `fusion_core`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
