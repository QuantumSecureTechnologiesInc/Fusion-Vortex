# Fusion Runtime Core - User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Core Concepts](#core-concepts)
4. [Getting Started](#getting-started)
5. [Working with Tensors](#working-with-tensors)
6. [Quantum Computing](#quantum-computing)
7. [High-Frequency Trading](#high-frequency-trading)
8. [Performance Optimisation](#performance-optimisation)
9. [Troubleshooting](#troubleshooting)

## Introduction

Fusion is a revolutionary programming language runtime designed for hybrid Quantum/AI/Classical workloads. Unlike traditional runtimes that treat GPUs and QPUs as afterthoughts, Fusion's `fusion_runtime_core` provides first-class support for heterogeneous computing.

### Why Fusion Runtime Core?

- **10x Faster**: Sub-10μs latency for HFT applications (vs 100μs with Tokio)
- **Zero-Copy**: Eliminates 30-40% overhead in tensor operations
- **QoS-Aware**: Predictable execution for latency-sensitive workloads
- **Unified**: Single API for CPU/GPU/QPU programming

## Installation

### Prerequisites

```bash
# Rust 1.75+
rustc --version

# LLVM (for quantum compilation)
llvm-config --version

# Optional: CUDA for NVIDIA GPUs
nvidia-smi

# Optional: Metal for Apple Silicon
# (Pre-installed on macOS)
```

### Quick Install

```bash
cargo add fusion_runtime_core fusion_core
```

### Full Ecosystem

```bash
cargo add fusion_runtime_core fusion_core fusion_ai_core fusion_finance fusion_quantum fusion_net
```

## Core Concepts

### FusionType

The fundamental type that can be Classical, Tensor, or Quantum:

```rust
use fusion_core::FusionType;

let classical = FusionType::int(42);
let tensor = Tensor::zeros([1024, 1024]).into();
let quantum = Qubit::new().into();
```

### Heterogeneous Scheduler

Three priority queues optimised for different workloads:

1. **Low-Jitter Queue** (High Priority): Financial, Quantum control
2. **High-Throughput Queue** (Normal): AI/ML, Batch processing
3. **External Device Queue**: QPU/TPU operations

### Zero-Copy Memory

Unified memory accessible from CPU, GPU, and QPU without copying:

```rust
let tensor = Tensor::zeros([1024, 1024])
    .device("cuda:0");  // Allocated once on GPU

// No copy when computing
let result = tensor.matmul(&tensor).await;
```

## Getting Started

### Hello, Fusion!

```rust
use fusion_runtime_core::Runtime;

fn main() {
    let runtime = Runtime::new();
    
    runtime.block_on(async {
        println!("Hello from Fusion Runtime!");
    });
}
```

### Using the Macro

```rust
use fusion_runtime_core;

#[fusion_runtime_core::main]
async fn main() {
    println!("Async main with Fusion!");
}
```

### Custom Configuration

```rust
use fusion_runtime_core::{Runtime, QoSMode};

let runtime = Runtime::builder()
    .enable_gpu()
    .enable_qpu()
    .enable_qos(QoSMode::UltraLowLatency)
    .worker_threads(16)
    .memory_pool_size(2 * 1024 * 1024 * 1024)  // 2GB
    .build();
```

## Working with Tensors

### Creating Tensors

```rust
use fusion_ai_core::Tensor;

// Zeros and ones
let zeros = Tensor::zeros([batch_size, channels, height, width]);
let ones = Tensor::ones([10, 10]);

// On specific device
let gpu_tensor = Tensor::zeros([1024, 1024])
    .device("cuda:0");

// With gradient tracking
let trainable = Tensor::ones([100, 100])
    .requires_grad(true);
```

### Matrix Operations

```rust
let a = Tensor::zeros([256, 512]);
let b = Tensor::zeros([512, 1024]);

// Zero-copy matmul on GPU
let c = a.matmul(&b).await;
println!("Result shape: {:?}", c.shape());
```

### Automatic Differentiation

```rust
use fusion_ai_core::{Tensor, Autodiff};

let mut autodiff = Autodiff::new();

let x = Tensor::ones([100, 10]).requires_grad(true);
let w = Tensor::ones([10, 1]).requires_grad(true);

// Forward pass
let y = x.matmul(&w).await;

// Backward pass
autodiff.backward(&y);
```

## Quantum Computing

### Single Qubit Operations

```rust
use fusion_quantum::Qubit;

#[fusion_runtime_core::main]
async fn main() {
    let mut qubit = Qubit::new();  // |0⟩
    
    qubit.hadamard();  // |+⟩ superposition
    
    let result = qubit.measure().await;
    println!("Measured: {}", result);  // 0 or 1 with 50% probability
}
```

### Quantum Circuits

```rust
use fusion_quantum::Circuit;

let mut circuit = Circuit::new(2);

circuit
    .h(0)           // Hadamard on qubit 0
    .cx(0, 1)       // CNOT (control=0, target=1)
    .measure(0)
    .measure(1);

let result = circuit.execute().await;
println!("Counts: {:?}", result.counts);
```

### Bell State

```rust
// Create maximally entangled Bell state
let mut circuit = Circuit::new(2);
circuit.h(0).cx(0, 1).measure(0).measure(1);

let result = circuit.execute().await;
// Expected: |00⟩ and |11⟩ with equal probability
```

## High-Frequency Trading

### Order Book

```rust
use fusion_finance::{OrderBook, Order};

#[fusion_runtime_core::main(qos = "ultra_low_latency")]
async fn main() {
    let book = OrderBook::new("BTC/USD");
    
    // Place orders (scheduled on low-jitter queue)
    let buy_order = Order::limit_buy(50000.0, 1.0);
    let order_id = book.place_order(buy_order).await;
    
    // Query best prices
    println!("Best bid: {:?}", book.best_bid());
    println!("Best ask: {:?}", book.best_ask());
}
```

### Market Making

```rust
use fusion_finance::{OrderBook, Order};

let book = OrderBook::new("ETH/USD");

// Continuous market making
loop {
    let mid_price = (book.best_bid().unwrap() + book.best_ask().unwrap()) / 2.0;
    let spread = 0.01;
    
    let buy = Order::limit_buy(mid_price - spread, 10.0);
    let sell = Order::limit_sell(mid_price + spread, 10.0);
    
    book.place_order(buy).await;
    book.place_order(sell).await;
}
```

## Performance Optimisation

### GPU Backend Selection

```rust
use fusion_runtime_core::Runtime;
use fusion_runtime_hal::GpuBackend;

let runtime = Runtime::builder()
    .enable_gpu()
    .gpu_backend(GpuBackend::Cuda)  // Force CUDA
    .build();
```

### QoS Modes

```rust
use fusion_runtime_core::QoSMode;

// Ultra-low latency (<10μs jitter)
let hft_runtime = Runtime::builder()
    .enable_qos(QoSMode::UltraLowLatency)
    .build();

// High throughput (AI/ML)
let ml_runtime = Runtime::builder()
    .enable_qos(QoSMode::HighThroughput)
    .build();
```

### Memory Pool Sizing

```rust
let runtime = Runtime::builder()
    .memory_pool_size(4 * 1024 * 1024 * 1024)  // 4GB pool
    .build();
```

## Troubleshooting

### GPU Not Detected

**Problem**: `fusion_runtime_core` can't find your GPU.

**Solution**:

```bash
# Check CUDA
nvidia-smi

# Check Metal (macOS)
system_profiler SPDisplaysDataType

# Verify Fusion detection
cargo run --example check_gpu
```

### QPU Authentication Failed

**Problem**: Cannot connect to QPU provider.

**Solution**:

```bash
# Set API key
export FUSION_QPU_API_KEY="your_ibm_quantum_api_key"

# Or use .env file
echo "FUSION_QPU_API_KEY=your_key" >> .env
```

### High Latency in HFT

**Problem**: Order processing latency > 10μs.

**Solution**:

```rust
// Enable ultra-low latency mode
let runtime = Runtime::builder()
    .enable_qos(QoSMode::UltraLowLatency)
    .worker_threads(1)  // Dedicated thread
    .build();
```

### Memory Pool Exhausted

**Problem**: `Out of memory in buffer pool` error.

**Solution**:

```rust
let runtime = Runtime::builder()
    .memory_pool_size(8 * 1024 * 1024 * 1024)  // Increase to 8GB
    .build();
```

## Next Steps

- Read the [Developer Guide](DeveloperGuide.md) for architecture details
- Explore the [API Reference](../references/API.md)
- Check out [Example Projects](../../examples/)
- Join our [Community Discord](https://discord.gg/fusion-lang)

## Support

- GitHub Issues: [github.com/QuantumSecureTechnologiesInc/Fusion/issues](https://github.com/QuantumSecureTechnologiesInc/Fusion/issues)
- Email: support@quantumsecuretech.com
- Documentation: [docs.fusion-lang.org](https://docs.fusion-lang.org)
