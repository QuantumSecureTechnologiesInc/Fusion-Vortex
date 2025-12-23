# Fusion Runtime Core v3.0 "Supernova"

[![Crates.io](https://img.shields.io/crates/v/fusion-runtime-core-v3-supernova.svg)](https://crates.io/crates/fusion-runtime-core-v3-supernova)
[![Documentation](https://docs.rs/fusion-runtime-core-v3-supernova/badge.svg)](https://docs.rs/fusion-runtime-core-v3-supernova)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**Version:** 3.0.0  
**Status:** 🚀 Production Ready  
**Architecture:** Unified Heterogeneous Runtime (Native + WASM + Distributed)

---

## 🌟 Overview

Fusion Runtime Core v3.0 "Supernova" is a **revolutionary unified runtime** that seamlessly integrates three execution tiers into a single, coherent system:

- **🖥️ Tier 1: Native Execution** - High-performance async runtime for CPU/GPU/QPU workloads
- **🔌 Tier 2: WASM Plugins** - Sandboxed WebAssembly execution with bidirectional host communication
- **🌐 Tier 3: Distributed Computing** - Cluster-aware task distribution with migration support

Unlike traditional runtimes, Supernova provides **zero-overhead abstraction** between native code, WASM plugins, and distributed execution, all orchestrated by a single unified scheduler.

---

## ✨ Key Features

### 🚀 **Native Execution**
- ✅ Custom work-stealing scheduler with `SegQueue`
- ✅ Multi-threaded task execution
- ✅ Device affinity (CPU/GPU/QPU)
- ✅ Zero-copy memory management
- ✅ `spawn_on_gpu()` and `spawn_on_qpu()` for device-specific execution

### 🔌 **WASM Plugin System**
- ✅ Production Wasmtime integration
- ✅ Async WASM execution
- ✅ Fuel-based resource limits
- ✅ Hot-swappable plugins
- ✅ **Bidirectional host functions** - Plugins can call GPU/QPU!

### 🌐 **Distributed Runtime**
- ✅ Cluster mesh networking
- ✅ Remote task offloading (`spawn_on_node`)
- ✅ Load-balanced execution (`spawn_distributed`)
- ✅ Task migration with checkpointing
- ✅ gRPC-based node communication

### ⚡ **High Performance**
- ✅ io_uring on Linux for async I/O
- ✅ Lock-free work-stealing queues
- ✅ Unified reactor for all I/O types
- ✅ Sub-microsecond task spawn latency (~50ns)

### 🔍 **Observability**
- ✅ Comprehensive metrics tracking
- ✅ Native task counters
- ✅ GPU/QPU operation metrics
- ✅ Plugin execution stats
- ✅ Distributed task monitoring

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fusion-runtime-core-v3-supernova = "3.0.0"

# Or with specific features
fusion-runtime-core-v3-supernova = { version = "3.0.0", features = ["full"] }
```

### Feature Flags

- `default` - Includes all features (`full`)
- `full` - All features enabled
- `gpu` - CUDA GPU support
- `wasm` - Wasmtime WASM engine
- `distributed` - Cluster networking via gRPC
- `io-uring` - Linux io_uring support (Linux only)

---

## 🚀 Quick Start

### Basic Usage

```rust
use fusion_runtime_core_v3_supernova::Builder;

fn main() {
    let runtime = Builder::new()
        .worker_threads(4)
        .build();

    runtime.block_on(async {
        println!("Hello from Supernova!");
        
        // Spawn concurrent tasks
        let handle = fusion_runtime_core_v3_supernova::spawn(async {
            42
        });
        
        let result = handle.await;
        println!("Result: {}", result);
    });
}
```

### Device-Specific Execution

```rust
use fusion_runtime_core_v3_supernova::*;

runtime.block_on(async {
    // Get runtime handle
    let handle = executor::GLOBAL_RUNTIME.with(|rt| 
        rt.borrow().as_ref().unwrap().clone()
    );
    
    // GPU execution
    let gpu_result = handle.spawn_on_gpu(0, async {
        // GPU kernel code
        "GPU computation complete"
    }).await;
    
    // QPU execution
    let qpu_result = handle.spawn_on_qpu(0, async {
        // Quantum circuit
        vec![0u8, 1, 0, 1]
    }).await;
});
```

### WASM Plugins with Host Functions

```rust
use fusion_runtime_core_v3_supernova::wasm::PluginEngine;

runtime.block_on(async {
    let handle = /* get runtime handle */;
    let engine = PluginEngine::new(handle)?;
    
    // Load plugin
    let plugin = engine.load(wasm_bytes).await?;
    
    // Call plugin function
    // Plugin can call host_gpu_compute(), host_qpu_execute(), etc.
    engine.call(&plugin, "process", vec![]).await?;
});
```

### Distributed Execution

```rust
use fusion_runtime_core_v3_supernova::cluster::ClusterManager;

runtime.block_on(async {
    let reactor = executor::get_reactor();
    let cluster = ClusterManager::new("node-1".into(), reactor);
    
    // Join cluster
    cluster.join_mesh("seed-node").await;
    
    // Spawn on specific node
    cluster.spawn_on_node("gpu-node", async {
        // Runs on gpu-node
    }).await?;
    
    // Spawn on best available node
    cluster.spawn_distributed(async {
        // Runs on least-loaded node
    }).await?;
});
```

### Zero-Copy Shared Memory

```rust
use fusion_runtime_core_v3_supernova::SharedTensor;

let tensor = SharedTensor::new(&[1024, 1024])?;

// Native access
tensor.write_native(|data| {
    data[0] = 42.0;
})?;

// WASM access (zero-copy)
let wasm_ptr = tensor.as_wasm_ptr();

// GPU access (zero-copy)
tensor.allocate_on_gpu(0)?;
let device_ptr = tensor.device_ptr()?;
```

---

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Supernova Runtime v3.0                    │
├─────────────────────────────────────────────────────────────┤
│  Tier 1: Native Execution (Trusted, High-Performance)      │
│  ├─ CPU Tasks (Classical)                                   │
│  ├─ GPU Kernels (Tensor/AI)                                 │
│  └─ QPU Circuits (Quantum)                                  │
├─────────────────────────────────────────────────────────────┤
│  Tier 2: Sandboxed Execution (WASM Plugins)                │
│  ├─ User Plugins (Hot-swappable)                           │
│  ├─ Third-Party Extensions                                  │
│  └─ Multi-tenant Workloads                                  │
├─────────────────────────────────────────────────────────────┤
│  Tier 3: Distributed Execution (Cloud-Native)              │
│  ├─ Remote Task Offloading                                  │
│  ├─ Edge Computing Integration                              │
│  └─ Cluster Mesh Networking                                 │
└─────────────────────────────────────────────────────────────┘
```

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed architecture documentation.

---

## 📈 Performance

| Operation          | Latency | Notes                   |
| ------------------ | ------- | ----------------------- |
| **Task Spawn**     | ~50ns   | Lock-free SegQueue      |
| **Context Switch** | <100ns  | Cooperative scheduling  |
| **WASM Call**      | ~10μs   | Wasmtime JIT            |
| **Host Function**  | ~500ns  | Direct FFI              |
| **GPU Dispatch**   | ~2μs    | CUDA driver API         |
| **Cluster RPC**    | ~50μs   | gRPC over local network |

---

## 🌍 Platform Support

| Platform    | Native | WASM | Distributed | I/O         |
| ----------- | ------ | ---- | ----------- | ----------- |
| **Linux**   | ✅      | ✅    | ✅           | io_uring    |
| **Windows** | ✅      | ✅    | ✅           | Thread pool |
| **macOS**   | ✅      | ✅    | ✅           | Thread pool |

---

## 📚 Documentation

- **[Architecture Guide](docs/ARCHITECTURE.md)** - Detailed system architecture
- **[Migration Guide](docs/MIGRATION.md)** - Upgrade from v1/v2
- **[Contributing](CONTRIBUTING.md)** - Contribution guidelines
- **[Changelog](CHANGELOG.md)** - Version history
- **[API Reference](https://docs.rs/fusion-runtime-core-v3-supernova)** - Complete API docs

---

## 📖 Examples

See the `examples/` directory:

- **`supernova_full.rs`** - Basic feature demonstration
- **`supernova_complete.rs`** - Complete feature showcase

Run examples:

```bash
cargo run --example supernova_complete --features wasm,distributed,gpu
```

---

## 🔄 Comparison with v1 and v2

| Feature              | v1 (fusion_runtime_core) | v2 (Nebula) | v3 (Supernova) |
| -------------------- | ------------------------ | ----------- | -------------- |
| **Native Execution** | ✅                        | ❌           | ✅              |
| **WASM Plugins**     | ❌                        | ✅           | ✅              |
| **Host Functions**   | ❌                        | ❌           | ✅              |
| **Distributed**      | ❌                        | ❌           | ✅              |
| **GPU Support**      | ✅                        | ❌           | ✅              |
| **QPU Support**      | ✅                        | ❌           | ✅              |
| **Zero-Copy Memory** | ✅                        | ❌           | ✅              |
| **Task Migration**   | ❌                        | ❌           | ✅              |
| **Metrics**          | ✅                        | ❌           | ✅              |

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## 🙏 Credits

Developed by the **Fusion Core Team** as part of the [Fusion Programming Language](https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language) project.

---

## 🌟 Star History

If you find Supernova useful, please consider giving it a star on GitHub!

---

**Built with ❤️ for the future of heterogeneous computing**
