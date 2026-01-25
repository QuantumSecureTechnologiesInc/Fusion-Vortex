# Changelog

All notable changes to Fusion Runtime Core v3.0 "Supernova" will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0] - 2025-12-22

### Added - Major Release

#### Tier 1: Native Execution

- **Multi-threaded executor** with work-stealing scheduler using `SegQueue`
- **`spawn()`** - Spawn async tasks on worker pool
- **`block_on()`** - Block on future completion
- **`spawn_on_gpu()`** - Device-specific GPU task spawning
- **`spawn_on_qpu()`** - Device-specific quantum task spawning
- **`gpu_kernel()`** - Direct GPU kernel execution
- **`qpu_circuit()`** - Direct quantum circuit execution
- **HyperRing Reactor** - Unified I/O multiplexer
  - Timer support (async sleep)
  - File I/O (io_uring on Linux, thread pool fallback)
  - Network I/O
  - GPU event polling
  - QPU completion polling

#### Tier 2: WASM Plugin System

- **Wasmtime integration** - Production WASM engine with async support
- **Plugin loading** - Compile and load WASM modules
- **Plugin execution** - Call WASM functions with fuel limits
- **Host functions** - WASM→Host communication
  - `host_log()` - Logging from plugins
  - `host_read_file()` / `host_write_file()` - File I/O
  - `host_get_state()` / `host_set_state()` - State management
  - `host_gpu_compute()` - GPU access from WASM
  - `host_qpu_execute()` - QPU access from WASM
  - `host_shared_memory()` - Shared memory access
- **HostState** - Context passed to plugins with runtime handle

#### Tier 3: Distributed Execution

- **ClusterManager** - Cluster mesh networking
- **`join_mesh()`** - Join cluster via seed node
- **`spawn_on_node()`** - Spawn task on specific node
- **`spawn_distributed()`** - Spawn on best available node
- **`migrate_task()`** - Migrate running task to another node
- **`checkpoint_task()`** - Checkpoint task state
- **`restore_task()`** - Restore from checkpoint

#### Shared Memory System

- **SharedTensor** - Zero-copy tensor sharing
- **`write_native()`** - Native write access
- **`read_native()`** - Native read access
- **`as_wasm_ptr()`** - WASM pointer (zero-copy)
- **`device_ptr()`** - GPU device pointer (zero-copy)
- **`allocate_on_gpu()`** - GPU memory allocation
- **SharedMemoryManager** - Manage shared memory regions

#### Metrics & Observability

- **RuntimeMetrics** - Comprehensive metrics collection
  - Native task counters
  - GPU/QPU operation counters
  - Plugin execution metrics
  - Distributed task metrics
  - Memory operation counters
  - I/O operation counters
- **MetricsSnapshot** - Point-in-time metrics view
- **`print_summary()`** - Human-readable metrics output

#### CUDA Driver

- **fusion-cuda-driver** crate - Direct CUDA FFI bindings
- **CudaDriver** - CUDA device management
- **Stream management** - CUDA stream creation and synchronization
- **Kernel launch** - GPU kernel execution support
- **Linux-only** - Full support on Linux, stub on Windows/macOS

#### Core Infrastructure

- **Builder pattern** - Ergonomic runtime configuration
- **Feature flags** - Optional GPU, WASM, distributed, io-uring
- **Error handling** - Comprehensive `FusionError` types
- **Async primitives** - `AsyncMutex` and other sync primitives
- **Task system** - Self-waking futures with `ArcWake`

### Documentation

- **README.md** - Comprehensive project overview
- **ARCHITECTURE.md** - Detailed architecture guide
- **MIGRATION.md** - Migration guide from v1/v2
- **CONTRIBUTING.md** - Contribution guidelines
- **Examples**:
  - `supernova_full.rs` - Basic feature demonstration
  - `supernova_complete.rs` - Complete feature showcase

### Platform Support

- **Linux** - Full support with io_uring
- **Windows** - Thread pool I/O, WASM, distributed
- **macOS** - Thread pool I/O, WASM, distributed

### Performance

- Task spawn: ~50ns (native), ~10μs (WASM)
- Context switch: <100ns
- Host function call: ~500ns
- GPU kernel dispatch: ~2μs
- Cluster RPC: ~50μs (local network)

## [2.0.0] - Previous (Nebula)

### Features

- WASM-only execution
- Basic plugin system
- gRPC server
- Fuel-based limits

## [1.0.0] - Previous (v1)

### Features

- Native execution only
- Heterogeneous scheduler
- GPU/QPU support
- Memory management

---

## Upgrade Guide

See [MIGRATION.md](docs/MIGRATION.md) for detailed upgrade instructions.

## License

MIT OR Apache-2.0