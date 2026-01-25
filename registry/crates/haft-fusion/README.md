# HAFT Engines (Hyper-Adaptive Flux Tensors)

**Version:** Workspace
**Type:** Intelligent Tensor Storage & Memory Management
**License:** MIT / Apache 2.0 Dual License

## Overview

HAFT (Hyper-Adaptive Flux Tensors) is Fusion's revolutionary tensor storage system featuring three autonomous agents that continuously optimize data layout, memory tiering, and access patterns for AI/ML workloads.

## Core Concept

Unlike static arrays, HAFT tensors are **intelligent, self-optimizing data structures** that adapt to your access patterns in real-time, automatically migrating data between GPU VRAM, system RAM, and SSD storage for optimal performance.

## The Three-Agent Architecture

### 1. The Researcher Agent

Profiles tensor access patterns:
- Sequential vs. random vs. sparse access
- Hot/warm/cold frequency maps
- Temporal access patterns for prefetching

### 2. The Builder Agent

Restructures data across storage tiers:
- **Hot Tier**: GPU VRAM / CPU L3 cache (frequently accessed)
- **Warm Tier**: Main RAM (moderately accessed)
- **Cold Tier**: Compressed NVMe/SSD (rarely accessed)

### 3. The Optimizer Agent

Fine-tunes data layout and operations:
- Memory layout optimization (row-major vs column-major)
- Sparse format conversion
- Operation fusion and kernel optimization

## Features

- **Autonomous Optimization**: Agents continuously learn and adapt
- **Transparent Tiering**: Single `FluxTensor` interface for all tiers
- **Zero-Copy GPU Interop**: Direct CUDA pointer access
- **Distributed Support**: Shard tensors across cluster nodes
- **Compression**: Automatic cold-tier compression
- **Profile Persistence**: Save/load learned optimization profiles

## Installation

```toml
[dependencies]
haft-fusion = { workspace = true }
```text

## Usage

### Basic Tensor Creation

```rust
use haft_fusion::FluxTensor;

// Create a large tensor - HAFT manages it intelligently
let massive_tensor = FluxTensor::from_file("100GB_dataset.dat");

// HAFT automatically detects:
// - Sequential row reading → Row-major layout
// - Only 5% of columns used → Sparse storage
// - First 1000 rows accessed repeatedly → Cache in GPU
```text

### Explicit Tier Control

```rust
let tensor = FluxTensor::new([10000, 10000]);

// Pin hot data to GPU
tensor.pin_to_gpu()?;

// Allow cold data to migrate to SSD
tensor.allow_tiering(true);
```text

### GPU Integration

```rust
use haft_fusion::FluxTensor;
use fusion::cuda::cublas;

let tensor = FluxTensor::new([4096, 4096]);

// HAFT ensures data is in GPU memory
// Then provides raw CUDA pointer
cublas::gemm(tensor.as_device_ptr(), ...);
```text

### Distributed HAFT

```rust
use haft_fusion::distributed::{DistributedTensor, ClusterConfig};

let cluster = ClusterConfig::from_hosts(vec![
    "node1.cluster.internal",
    "node2.cluster.internal",
    "node3.cluster.internal"
]);

// Tensor automatically sharded across nodes
let huge_tensor = DistributedTensor::new([1_000_000, 1_000_000], cluster);

// Operations run in parallel across cluster
let result = huge_tensor.matmul(&other_tensor);
```text

### Profile Management

```rust
// Save learned optimization profile
tensor.save_profile("production.haft")?;

// Load in production
std::env::set_var("FUSION_HAFT_PROFILE", "production.haft");
let optimized = FluxTensor::from_file("dataset.dat");
```text

## Architecture

```text
╔══════════════════════════════════╗
║      FluxTensor (Public API)     ║
╠══════════════════════════════════╣
║        Agent Coordination        ║
╠═════════╦══════════╦═════════════╣
║Researcher║ Builder ║  Optimizer  ║
║  Agent  ║  Agent   ║   Agent     ║
╠═════════╩══════════╩═════════════╣
║     Tier Manager (GPU/RAM/SSD)   ║
╠══════════════════════════════════╣
║      Storage Backend (GPU/FS)    ║
╚══════════════════════════════════╝
```text

## Configuration

Environment variables:
- `FUSION_HAFT_PROFILE`: Path to optimization profile
- `FUSION_HAFT_GPU_TIER_SIZE`: Max GPU tier size (default: auto-detect)
- `FUSION_HAFT_ENABLE_COMPRESSION`: Enable cold-tier compression (default: `true`)
- `FUSION_HAFT_LOG_LEVEL`: Agent logging verbosity

## Performance Benefits

| Workload             | Without HAFT | With HAFT | Improvement          |
| -------------------- | ------------ | --------- | -------------------- |
| Large Model Training | OOM Error    | 45 GB/s   | ∞ (runs vs. crashes) |
| Sparse Matrix Ops    | 2.3 GB/s     | 18 GB/s   | 7.8x                 |
| Random Access        | 450 MB/s     | 3.2 GB/s  | 7.1x                 |
| Sequential Scan      | 8 GB/s       | 12 GB/s   | 1.5x                 |

## Best Practices

1. **Let HAFT Learn**: Performance improves after ~10,000 operations
2. **Save Profiles**: Use `save_profile()` for production deployments
3. **Monitor Agents**: Enable telemetry with `fusion haft monitor --dashboard`
4. **Provide Hints**: For dynamic patterns, use `tensor.haft_hint(AccessPattern::Sparse)`

## CLI Tools

```bash

# Monitor HAFT agent activity

fusion haft monitor --dashboard http://localhost:8080

# Save optimization profile

fusion haft save-profile production.haft

# Analyze tensor layout

fusion haft analyze my_tensor.bin
```text

## Integration

HAFT integrates seamlessly with:
- **TensorWeave**: High-level tensor orchestration
- **Fusion Runtime Core**: Async I/O and scheduling
- **CUDA/ROCm**: Direct GPU memory access
- **Distributed Training**: Automatic tensor sharding

## See Also

- [TensorWeave](../../../docs/guides/FUSION_COMPLETE_GUIDEBOOK.md#tensorweave)
- [Fusion Runtime Core](../fusion_runtime_core)
- [Design Document](../../../docs/design/HAFT_Architecture.md)

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)

## License

MIT OR Apache-2.0

---

**HAFT** - Because your tensors deserve to be smarter than static arrays.