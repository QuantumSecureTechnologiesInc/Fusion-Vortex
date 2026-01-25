# Fusion Runtime Core - Quick Start Guide

## Installation

### Prerequisites

- Rust 1.75 or later
- LLVM 17+ (for quantum compilation)
- CUDA 12.0+ (optional, for NVIDIA GPU support)
- Metal SDK (optional, for macOS GPU support)

### Build from Source

```bash

# Clone the repository

git clone https://github.com/QuantumSecureTechnologiesInc/Fusion.git
cd Fusion

# Build in release mode

cargo build --workspace --release

# Run tests to verify installation

cargo test --workspace
```text

## Basic Usage

### Example 1: Hybrid Classical/Tensor Computation

```rust
use fusion_core::FusionType;
use fusion_runtime_core::Runtime;
use fusion_ai_core::Tensor;

#[fusion_runtime_core::main]

async fn main() {
    let runtime = Runtime::new()
        .enable_gpu()
        .enable_qos_scheduling();

    // Create a tensor on GPU
    let tensor: FusionType = Tensor::zeros([1024, 1024])
        .device("cuda:0")
        .into();

    // Perform zero-copy computation
    let result = tensor.matmul(&tensor).await;

    println!("Result shape: {:?}", result.shape());
}
```text

### Example 2: Quantum Circuit Execution

```rust
use fusion_quantum::Qubit;
use fusion_runtime_core::Runtime;

#[fusion_runtime_core::main]

async fn main() {
    let runtime = Runtime::new()
        .enable_qpu("ibm_quantum");

    let qubit = Qubit::new();
    qubit.hadamard();
    qubit.measure().await;
}
```text

### Example 3: High-Frequency Trading

```rust
use fusion_finance::OrderBook;
use fusion_runtime_core::Runtime;

#[fusion_runtime_core::main(qos = "low_latency")]

async fn main() {
    let runtime = Runtime::new()
        .enable_low_jitter_queue()
        .enable_dpdk_networking();

    let order_book = OrderBook::new("BTC/USD");

    // Sub-10μs order processing
    order_book.place_order(Order::limit_buy(50000.0, 1.0)).await;
}
```text

## Configuration

Create a `fusion.toml` configuration file:

```toml
[runtime]
scheduler = "heterogeneous"
qos_enabled = true

[memory]
zero_copy = true
unified_memory = true
buffer_pool_size = "1GB"

[hardware]
gpu_backend = "cuda"  # or "metal", "vulkan"
enable_qpu = true
qpu_provider = "ibm_quantum"

[network]
low_latency_mode = true
dpdk_enabled = false  # Requires root privileges
```text

## Performance Tuning

### For AI/ML Workloads

```bash
FUSION_GPU_THREADS=8 \
FUSION_TENSOR_CACHE=2GB \
cargo run --release
```text

### For Financial Applications

```bash
FUSION_QOS_MODE=ultra_low_latency \
FUSION_JITTER_BUDGET=5us \
cargo run --release
```text

### For Quantum Computing

```bash
FUSION_QPU_PROVIDER=ibm_quantum \
FUSION_QUBIT_POOLING=enabled \
cargo run --release
```text

## Next Steps

- Read the [User Guide](docs/guides/UserGuide.md) for detailed tutorials
- Explore the [API Documentation](docs/references/API.md)
- Check out [Example Projects](examples/)
- Join our [Community Discord](https://discord.gg/fusion-lang)

## Troubleshooting

### GPU Not Detected

```bash

# Verify CUDA installation

nvidia-smi

# Check Fusion GPU detection

cargo run --example check_gpu
```text

### QPU Connection Issues

```bash

# Verify QPU credentials

export FUSION_QPU_API_KEY="your_key_here"

# Test connection

cargo run --example test_qpu_connection
```text

## Support

- Documentation: [docs/](docs/)
- Issues: [GitHub Issues](https://github.com/QuantumSecureTechnologiesInc/Fusion/issues)
- Email: support@quantumsecuretech.com