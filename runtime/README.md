# Fusion Programming Language - Runtime Core

**Version:** 0.3.0 (Alpha)

## Overview

Fusion is a next-generation programming language designed for hybrid Quantum/AI/Classical workloads. At its heart lies `fusion_runtime_core`, a custom, bespoke runtime engine that replaces traditional async runtimes (like Tokio) with a heterogeneous scheduler optimised for:

- **Quantum Processing Units (QPU)**
- **Graphics Processing Units (GPU)** 
- **Tensor Processing Units (TPU)**
- **Classical CPU threads**

## What's New in v0.3.0

### 🧠 Cortex Engine (AI Scheduler)
The Cortex is the "brain" of the Fusion scheduler, using machine learning to predict optimal device placement for tasks:
- Intelligent cost prediction for CPU vs GPU vs QPU
- HFT Guard: Critical tasks always run on CPU with minimal jitter
- Online learning from execution logs

### ⚙️ Hardware Abstraction Layer (fusion_hal)
New low-level hardware interfaces:
- **CUDA Backend**: Native CUDA kernel execution via FFI
- **Pinned Thread Pools**: Work-stealing with core affinity for HFT
- **Unified Allocator**: CUDA-aware allocator for zero-copy GPU transfers

### ⚛️ Quantum Error Mitigation (QEM)
Automatic stabilisation of quantum circuits:
- **Dynamical Decoupling**: XY4 pulse insertion during idle periods
- **Zero-Noise Extrapolation**: Multi-scale execution for noise estimation
- **Circuit Validation**: Automatic depth checking for decoherence risk

## Key Features

### 🚀 Heterogeneous Scheduling
- QoS-aware task prioritisation
- Low-jitter queue for latency-sensitive financial/quantum operations
- High-throughput queue for AI/ML gradient calculations
- External device queue for QPU/TPU job submissions

### 💾 Zero-Copy Memory Management
- Unified memory access across CPU/GPU/QPU
- Device-aware buffer pooling
- Predictable tensor placement
- Qubit memory model with decoherence tracking

### ⚡ Three-Plane Architecture
- **Classical Plane**: HFT logic, networking, file I/O
- **Accelerator Plane**: GPU/TPU tensor operations
- **Quantum Plane**: QPU interfacing with QEM middleware

## Project Structure

```text
runtime/
├── crates/
│   ├── fusion_core/              # Core FusionType abstraction
│   ├── fusion_runtime_core/      # Main runtime orchestrator
│   ├── fusion_runtime_scheduler/ # Heterogeneous scheduler
│   ├── fusion_runtime_mem_mgr/   # Zero-copy memory manager
│   ├── fusion_runtime_hal/       # Hardware abstraction layer
│   ├── fusion_cortex/            # ⭐ AI Scheduler (v0.3.0)
│   ├── fusion_hal/               # ⭐ CUDA/Pinned Pools (v0.3.0)
│   ├── fusion_ai_core/           # AI/ML with autodiff engine
│   ├── fusion_finance/           # Financial computing (HFT)
│   ├── fusion_net/               # High-performance networking
│   ├── fusion_quantum/           # Quantum primitives + QEM
│   ├── fusion_traits/            # Core traits for interweaving
│   ├── fusion_tensor_core/       # Tensor operations
│   └── fusion_quantum_core/      # Quantum primitives
├── docs/                         # Comprehensive documentation
├── examples/                     # Usage examples
└── .github/workflows/            # CI/CD configuration
```

## Quick Start

```bash
# Build all crates (simulation mode)
cargo build --workspace --release --features simulation

# Run tests
cargo test --workspace --features simulation

# Run benchmarks
cargo bench --workspace --features simulation
```

### Feature Flags

| Flag         | Description                                              |
| ------------ | -------------------------------------------------------- |
| `simulation` | CPU simulation for Quantum/GPU (default for development) |
| `gpu`        | Enable CUDA/HIP compilation (requires CUDA toolkit)      |
| `quantum`    | Enable QPU bridge (requires Python/Qiskit)               |
| `hft`        | Enable kernel-bypass networking (requires root)          |

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- [Developer Guide](docs/guides/DEVELOPER_GUIDE.md) - Architecture and build instructions
- [Technical Sheet](docs/guides/TECHNICAL_SHEET.md) - System requirements and specs
- [Product Info Sheet](docs/guides/PRODUCT_INFO_SHEET.md) - Commercial overview

## Performance Characteristics

| Metric         | Traditional (Tokio) | Fusion v0.3.0      |
| -------------- | ------------------- | ------------------ |
| QPU Scheduling | ❌ Not Supported     | ✅ Native + QEM     |
| GPU Memory     | Standard Allocator  | Zero-Copy + Pinned |
| Latency (HFT)  | ~100μs              | <10μs              |
| Tensor Ops     | Copy-based          | Zero-Copy          |
| Event Loop     | ~1μs                | <400ns             |
| AI Scheduling  | Manual              | ✅ Cortex Engine    |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Application                         │
├─────────────────────────────────────────────────────────────┤
│                    Fusion Runtime Core                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────────┐│
│  │   Cortex    │ │  Scheduler  │ │    Memory Manager       ││
│  │  (AI Sched) │ │  (QoS-aware)│ │    (Zero-Copy)          ││
│  └─────────────┘ └─────────────┘ └─────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│                Hardware Abstraction Layer                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────────────────┐│
│  │  CUDA   │ │ Pinned  │ │ Unified │ │        QEM          ││
│  │ Backend │ │  Pools  │ │ Alloc   │ │    (Quantum EM)     ││
│  └─────────┘ └─────────┘ └─────────┘ └─────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│  Classical Plane  │  Accelerator Plane  │  Quantum Plane    │
│   (CPU/Network)   │    (GPU/TPU)        │      (QPU)        │
└─────────────────────────────────────────────────────────────┘
```

## Licence

Dual-licensed under MIT OR Apache-2.0

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

### Quick Contributing Guide

1. **Safety First**: No `unsafe` outside `fusion_hal`
2. **Zero-Copy**: Avoid `memcpy` unless essential
3. **Documentation**: All public APIs need docstrings
4. **Testing**: Unit tests + integration tests required

---

**Built with ❤️ by Quantum Secure Technologies Inc.**

*Bridging Classical, AI, and Quantum Computing*
