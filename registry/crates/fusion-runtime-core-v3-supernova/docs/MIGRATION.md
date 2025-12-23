# Migration Guide: v1/v2 → v3.0 Supernova

## Overview

This guide helps you migrate from:
- **v1 (`fusion_runtime_core`)** → v3 Supernova
- **v2 Nebula (`fusion-runtime-core-v2-nebula`)** → v3 Supernova

## From v1 to v3

### API Compatibility

**Good news**: v3 maintains API compatibility with v1 for native execution!

#### Before (v1)
```rust
use fusion_runtime_core::Runtime;

let runtime = Runtime::builder()
    .enable_gpu()
    .enable_qpu()
    .build();

runtime.block_on(async {
    // Your code
});
```

#### After (v3)
```rust
use fusion_runtime_core_v3_supernova::Builder;

let runtime = Builder::new()
    .enable_gpu()
    .enable_wasm()        // NEW!
    .enable_distributed() // NEW!
    .build();

runtime.block_on(async {
    // Same code works!
});
```

### New Capabilities in v3

#### 1. Device-Specific Spawning

```rust
// v1: No device-specific spawning
runtime.spawn(future).await;

// v3: Device-specific spawning
runtime_handle.spawn_on_gpu(0, future).await;
runtime_handle.spawn_on_qpu(0, future).await;
```

#### 2. WASM Plugins

```rust
// v1: No plugin support

// v3: Full WASM plugin system
let engine = PluginEngine::new(runtime_handle)?;
let plugin = engine.load(wasm_bytes).await?;
engine.call(&plugin, "process", vec![]).await?;
```

#### 3. Distributed Execution

```rust
// v1: No distributed support

// v3: Cluster-aware execution
cluster.spawn_on_node("gpu-node", future).await;
cluster.spawn_distributed(future).await;
```

## From v2 Nebula to v3

### Enhanced Plugin System

#### Before (v2)
```rust
use fusion_runtime_core_v2_nebula::WasmEngine;

let engine = WasmEngine::new()?;
let result = engine.execute(&wasm_bytes, &input)?;
```

#### After (v3)
```rust
use fusion_runtime_core_v3_supernova::wasm::PluginEngine;

let engine = PluginEngine::new(runtime_handle)?;
let plugin = engine.load(&wasm_bytes).await?;

// Plugins can now call host functions!
engine.call(&plugin, "process", vec![]).await?;
```

### Host Functions (NEW in v3)

v2 plugins were isolated. v3 plugins can call back to the host:

```rust
// In your WASM plugin (Rust → WASM)
extern "C" {
    fn host_gpu_compute(device_id: i32, data_ptr: *const u8, len: i32) -> i32;
    fn host_qpu_execute(device_id: i32, circuit_ptr: *const u8, len: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn process() {
    unsafe {
        // Call GPU from WASM!
        host_gpu_compute(0, data.as_ptr(), data.len() as i32);
    }
}
```

### Native Execution (NEW in v3)

v2 was WASM-only. v3 adds native execution:

```rust
// v2: Only WASM execution

// v3: Native + WASM + Distributed
runtime.block_on(async {
    // Native tasks
    spawn(async { /* CPU work */ }).await;
    
    // GPU tasks
    spawn_on_gpu(0, async { /* GPU work */ }).await;
    
    // WASM plugins
    engine.call(&plugin, "func", vec![]).await;
    
    // Distributed tasks
    cluster.spawn_on_node("node-1", async { /* ... */ }).await;
});
```

## Feature Mapping

| Feature              | v1  | v2 Nebula | v3 Supernova |
| -------------------- | --- | --------- | ------------ |
| **Native Execution** | ✅   | ❌         | ✅            |
| **`spawn()`**        | ✅   | ❌         | ✅            |
| **`block_on()`**     | ✅   | ❌         | ✅            |
| **`spawn_on_gpu()`** | ❌   | ❌         | ✅            |
| **`spawn_on_qpu()`** | ❌   | ❌         | ✅            |
| **WASM Plugins**     | ❌   | ✅         | ✅            |
| **Host Functions**   | ❌   | ❌         | ✅            |
| **Shared Memory**    | ✅   | ❌         | ✅            |
| **Distributed**      | ❌   | ❌         | ✅            |
| **Metrics**          | ✅   | ❌         | ✅            |

## Step-by-Step Migration

### Step 1: Update Dependencies

```toml
# Before
[dependencies]
fusion_runtime_core = "0.2.0"  # v1
# OR
fusion-runtime-core-v2-nebula = "2.0.0"  # v2

# After
[dependencies]
fusion-runtime-core-v3-supernova = { version = "3.0.0", features = ["full"] }
```

### Step 2: Update Imports

```rust
// Before (v1)
use fusion_runtime_core::Runtime;

// Before (v2)
use fusion_runtime_core_v2_nebula::WasmEngine;

// After (v3)
use fusion_runtime_core_v3_supernova::{Builder, spawn};
```

### Step 3: Update Runtime Creation

```rust
// Before (v1)
let runtime = Runtime::new();

// After (v3)
let runtime = Builder::new()
    .worker_threads(4)
    .enable_gpu()
    .enable_wasm()
    .build();
```

### Step 4: Migrate Plugin Code (v2 → v3)

```rust
// Before (v2)
let engine = WasmEngine::new()?;
engine.execute(&wasm, &input)?;

// After (v3)
let runtime_handle = /* get from GLOBAL_RUNTIME */;
let engine = PluginEngine::new(runtime_handle)?;
let plugin = engine.load(&wasm).await?;
engine.call(&plugin, "entry_point", vec![]).await?;
```

### Step 5: Add New Features

```rust
// Take advantage of new v3 features
runtime.block_on(async {
    // Device-specific execution
    let gpu_result = runtime_handle.spawn_on_gpu(0, async {
        // GPU kernel
    }).await;
    
    // Shared memory
    let tensor = SharedTensor::new(&[1024, 1024])?;
    tensor.write_native(|data| { /* ... */ })?;
    
    // Distributed execution
    cluster.spawn_distributed(async {
        // Runs on best available node
    }).await;
});
```

## Breaking Changes

### v1 → v3

1. **Builder Pattern**: `Runtime::new()` → `Builder::new().build()`
2. **Module Paths**: Some internal modules reorganized
3. **Feature Flags**: New features require opt-in

### v2 → v3

1. **Async API**: `execute()` is now `async`
2. **Plugin Loading**: Separate `load()` and `call()` steps
3. **Host State**: Plugins now receive `HostState` context

## Compatibility Layer

If you need gradual migration, you can use both:

```rust
// Use v1 for existing code
use fusion_runtime_core as v1;

// Use v3 for new features
use fusion_runtime_core_v3_supernova as v3;

// Gradually migrate module by module
```

## Performance Improvements

- **v1 → v3**: Same performance for native execution
- **v2 → v3**: ~2x faster plugin execution (Wasmtime optimizations)
- **New in v3**: Zero-copy shared memory eliminates data copying

## Support

For migration issues:
- Check examples in `examples/` directory
- Review architecture docs in `docs/ARCHITECTURE.md`
- File issues on GitHub

## Recommended Migration Path

1. **Week 1**: Update dependencies, test basic functionality
2. **Week 2**: Migrate core runtime usage
3. **Week 3**: Add device-specific spawning
4. **Week 4**: Integrate WASM plugins with host functions
5. **Week 5**: Enable distributed execution
6. **Week 6**: Add metrics and observability

**Total estimated migration time**: 4-6 weeks for large projects
