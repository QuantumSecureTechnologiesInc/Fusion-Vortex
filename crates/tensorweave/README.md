# TensorWeave

**Production-Grade Interwoven Tensor Flow Engine**

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/yourorg/tensorweave)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## Overview

TensorWeave is a production-ready tensor processing engine designed for secure, observable, and resilient machine learning pipelines. It provides a pluggable architecture for tensor transformations with built-in security checks, numerical stability, SVD optimization, and learning capabilities.

## Key Features

* **🔒 Security-First**: Integrity checks prevent corrupted data from propagating
* **📊 Numerical Stability**: Automatic NaN/Inf detection and correction
* **⚡ SVD Optimization**: Matrix compression using Singular Value Decomposition
* **🧠 Learning Support**: SGD-style gradient updates for training pipelines
* **🔍 Observable**: Structured JSON logging with distributed tracing
* **⚙️ Configurable**: Environment variable-based configuration
* **🚀 Parallel Processing**: Rayon-based parallelism for high throughput

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tensor_weave = { path = "../tensorweave" }
```text

### Basic Usage

```rust
use tensor_weave::{TensorData, TensorWeaveEngine, FlowProcessor};
use tensor_weave::processors::{SecureProcessor, StabilizeProcessor};

fn main() -> anyhow::Result<()> {
    // Create engine
    let mut engine = TensorWeaveEngine::new(30);

    // Add processors
    engine.add_processor(Box::new(SecureProcessor));
    engine.add_processor(Box::new(StabilizeProcessor));

    // Create sample tensor
    let tensor = TensorData::new("sample", vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    // Process batch
    let results = engine.run_batch_sync(vec![tensor]);

    Ok(())
}
```text

### CLI Usage

#### Run Benchmark

```bash
tensor_weave benchmark --batch-size 10 --matrix-size 100
```text

#### Process JSON File

```bash
tensor_weave process --input data.json --output results.json
```text

## Configuration

Configure via environment variables with `TENSOR__` prefix:

```bash

# SVD compression ratio

export TENSOR__OPTIMIZATION__SVD_KEEP_RATIO=0.95

# Learning rate for SGD

export TENSOR__LEARNING__LEARNING_RATE=0.001

# Momentum for optimizer

export TENSOR__LEARNING__MOMENTUM=0.9

# Logging level

export TENSOR__SERVER__LOG_LEVEL=debug
```text

## Processors

### SecureProcessor

Validates tensor integrity:
- Data length matches shape product
- No zero dimensionsall values are finite (no NaN/Inf)

### StabilizeProcessor

Ensures numerical stability:
- Replaces NaN with 0.0
- Clamps infinities to ±1e10
- Adds metadata about stabilization

### SvdOptimizeProcessor

Compresses tensors using SVD:
- Decomposes matrix into U, Sigma, V^T
- Truncates based on energy threshold
- Reconstructs approximated matrix

### SgdLearnProcessor

Applies gradient descent updates:
- Simulates training steps
- Configurable learning rate
- L2 regularization effect

## Architecture

```text
┌─────────────────────┐
│   TensorData        │
│  - id, trace_id     │
│  - shape, data      │
│  - metadata         │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ TensorWeaveEngine   │
│ ┌─────────────────┐ │
│ │ SecureProcessor │ │
│ └─────────────────┘ │
│ ┌─────────────────┐ │
│ │SvdOptimize...   │ │
│ └─────────────────┘ │
│ ┌─────────────────┐ │
│ │SgdLearnProc...  │ │
│ └─────────────────┘ │
│ ┌─────────────────┐ │
│ │StabilizeProc... │ │
│ └─────────────────┘ │
└─────────────────────┘
           │
           ▼
    ┌──────────┐
    │ Results  │
    └──────────┘
```text

## Testing

Run the test suite:

```bash
cargo test --lib
```text

Individual processor tests:

```bash
cargo test secure
cargo test stabilize
cargo test optimize
cargo test learn
```text

## Performance

- **Parallel Processing**: Utilizes Rayon for multi-core execution
- **Zero-Copy**: Efficient ndarray operations
- **Batch Optimized**: Processes multiple tensors in parallel

Typical performance (100x100 matrices, batch of 10):
- **Secure + Stabilize**: ~5ms
- **+ SVD Optimization**: ~50ms
- **+ Learning Step**: ~60ms

## Dependencies

### Core

- `ndarray` - Tensor operations
- `ndarray-linalg` - Linear algebra (requires OpenBLAS/LAPACK)
- `rayon` - Parallel processing

### Utility

- `serde` + `serde_json` - Serialization
- `config` - Configuration management
- `uuid` - Unique identifiers
- `clap` - CLI parsing
- `tracing` - Structured logging

## System Requirements

### Development

- Rust 1.70+
- OpenBLAS/LAPACK (for SVD functionality)

### Linux (Recommended)

```bash

# Ubuntu/Debian

sudo apt-get install libopenblas-dev liblapack-dev

# Fedora

sudo dnf install openblas-devel lapack-devel
```text

### macOS

```bash
brew install openblas lapack
```text

### Windows

SVD functionality requires OpenBLAS installation. Use Docker for full functionality.

## Docker

Production deployment:

```dockerfile
FROM rust:1.70 as builder
RUN apt-get update && apt-get install -y libopenblas-dev liblapack-dev
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libopenblas0 liblapack3
COPY --from=builder /app/target/release/tensor_weave /usr/local/bin/
CMD ["tensor_weave", "benchmark"]
```text

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Documentation

- **User Guide**: See `docs/guides/TensorWeave_User_Guide.md`
- **Developer Guide**: See `docs/guides/TensorWeave_Developer_Guide.md`
- **Technical Sheet**: See `docs/guides/TensorWeave_Technical_Sheet.md`
- **Product Info**: See `docs/guides/TensorWeave_Product_Info_Sheet.md`

## Support

For issues and questions:
- GitHub Issues: [Report a bug](https://github.com/yourorg/tensorweave/issues)
- Documentation: [Full docs](https://tensorweave.io/docs)
- Community: [Discord](https://discord.gg/tensorweave)

---

**Built with ❤️ by the TensorWeave Engineering Team**