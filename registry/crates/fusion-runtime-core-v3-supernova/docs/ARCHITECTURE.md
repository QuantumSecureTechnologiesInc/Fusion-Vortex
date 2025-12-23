# Fusion Runtime Core v3.0 "Supernova" - Architecture Guide

## Overview

Supernova is a unified heterogeneous runtime that seamlessly integrates three execution tiers:

1. **Tier 1: Native Execution** - High-performance CPU/GPU/QPU tasks
2. **Tier 2: WASM Plugins** - Sandboxed, hot-swappable modules
3. **Tier 3: Distributed Computing** - Cluster-wide task distribution

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    Application Layer                            │
│  (User Code using Builder API, spawn(), block_on())            │
└────────────────────┬────────────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────────────┐
│                  Runtime Core (lib.rs)                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Builder    │  │ JoinHandle   │  │ Device Enum  │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────────┬────────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
┌───────▼──────┐ ┌──▼────────┐ ┌─▼──────────┐
│   Executor   │ │  Reactor  │ │   Task     │
│              │ │ (HyperRing)│ │  System    │
│ - Workers    │ │           │ │            │
│ - SegQueue   │ │ - SQ/CQ   │ │ - ArcWake  │
│ - block_on() │ │ - Polling │ │ - Future   │
└──────┬───────┘ └─────┬─────┘ └─────┬──────┘
       │               │             │
       └───────┬───────┴─────┬───────┘
               │             │
    ┌──────────▼─────────────▼──────────┐
    │     Three-Tier Execution          │
    ├───────────────────────────────────┤
    │  Tier 1: Native (device.rs)      │
    │  - spawn_on_gpu()                 │
    │  - spawn_on_qpu()                 │
    │  - Direct hardware access         │
    ├───────────────────────────────────┤
    │  Tier 2: WASM (wasm.rs)          │
    │  - Wasmtime engine                │
    │  - Host functions                 │
    │  - Shared memory access           │
    ├───────────────────────────────────┤
    │  Tier 3: Distributed (cluster.rs)│
    │  - spawn_on_node()                │
    │  - Task migration                 │
    │  - Checkpointing                  │
    └───────────────────────────────────┘
```

## Core Components

### 1. Executor (`executor.rs`)

The executor manages task execution across multiple worker threads:

- **RuntimeHandle**: Thread-local handle for spawning tasks
- **Runtime**: Main runtime with worker pool
- **Work-Stealing**: Uses `SegQueue` for lock-free task distribution

**Key Methods:**
- `spawn()` - Spawn task on any worker
- `spawn_with_affinity()` - Spawn with device preference
- `block_on()` - Block until future completes

### 2. Reactor (`reactor.rs`)

The HyperRing reactor is a unified I/O multiplexer:

- **Submission Queue (SQ)**: Pending operations
- **Completion Queue (CQ)**: Completed operations
- **Event Registry**: Maps event IDs to wakers

**Supported Operations:**
- Sleep (timers)
- File I/O (io_uring on Linux)
- Network I/O
- GPU kernel launches
- QPU circuit execution

### 3. Task System (`task.rs`)

Tasks are self-waking futures:

- **Task**: Wraps a future with `ArcWake` implementation
- **Waker**: Automatically re-queues task when ready
- **Polling**: Tasks poll themselves until completion

### 4. Device Execution (`device.rs`)

Device-specific spawning functions:

```rust
// GPU execution
runtime.spawn_on_gpu(device_id, async {
    // GPU kernel code
}).await;

// QPU execution
runtime.spawn_on_qpu(device_id, async {
    // Quantum circuit
}).await;
```

### 5. Shared Memory (`shared_memory.rs`)

Zero-copy data sharing across all tiers:

```rust
let tensor = SharedTensor::new(&[1024, 1024])?;

// Native access
tensor.write_native(|data| data[0] = 42.0)?;

// WASM access (zero-copy pointer)
let ptr = tensor.as_wasm_ptr();

// GPU access (device pointer)
tensor.allocate_on_gpu(0)?;
let device_ptr = tensor.device_ptr()?;
```

### 6. Host Functions (`host_functions.rs`)

WASM plugins can call back to the host:

```rust
// In WASM plugin (Rust compiled to WASM)
extern "C" {
    fn host_log(level: i32, msg_ptr: *const u8, msg_len: i32);
    fn host_gpu_compute(device_id: i32, data_ptr: *const u8, len: i32) -> i32;
    fn host_qpu_execute(device_id: i32, circuit_ptr: *const u8, len: i32) -> i32;
}
```

### 7. Metrics (`metrics.rs`)

Comprehensive runtime observability:

```rust
let metrics = runtime.metrics().snapshot();
println!("Native tasks: {}", metrics.native_tasks_spawned);
println!("GPU kernels: {}", metrics.gpu_kernel_launches);
println!("QPU submissions: {}", metrics.qpu_submissions);
```

## Execution Flow

### Native Task Execution

```
User Code
   │
   ├─> spawn(future)
   │      │
   │      ├─> Create Task
   │      ├─> Push to SegQueue
   │      └─> Return JoinHandle
   │
   └─> Worker threads
          │
          ├─> Pop from SegQueue
          ├─> Poll task
          ├─> If Pending: register waker
          └─> If Ready: complete
```

### WASM Plugin Execution

```
User Code
   │
   ├─> PluginEngine::new()
   │      │
   │      ├─> Create Wasmtime engine
   │      ├─> Register host functions
   │      └─> Setup HostState
   │
   ├─> engine.load(wasm_bytes)
   │      │
   │      └─> Compile WASM module
   │
   └─> engine.call(plugin, "func", args)
          │
          ├─> Create Store with HostState
          ├─> Instantiate module
          ├─> Call function
          │      │
          │      └─> Plugin can call host functions
          │             │
          │             ├─> host_gpu_compute()
          │             ├─> host_qpu_execute()
          │             └─> host_shared_memory()
          │
          └─> Return result
```

### Distributed Task Execution

```
User Code
   │
   ├─> cluster.spawn_on_node("gpu-node", future)
   │      │
   │      ├─> Serialize future
   │      ├─> Send via gRPC to target node
   │      ├─> Remote node spawns task
   │      └─> Return JoinHandle (polls remote)
   │
   └─> cluster.migrate_task(task_id, "new-node")
          │
          ├─> Checkpoint task state
          ├─> Send checkpoint to new node
          ├─> Restore on new node
          └─> Cancel on old node
```

## Performance Characteristics

| Operation           | Latency | Notes                   |
| ------------------- | ------- | ----------------------- |
| Task spawn          | ~50ns   | Lock-free SegQueue      |
| Context switch      | <100ns  | Cooperative scheduling  |
| WASM call overhead  | ~10μs   | Wasmtime JIT            |
| Host function call  | ~500ns  | Direct FFI              |
| GPU kernel dispatch | ~2μs    | CUDA driver API         |
| QPU submission      | ~100μs  | Network latency         |
| Cluster RPC         | ~50μs   | gRPC over local network |

## Memory Model

### Zero-Copy Guarantees

1. **Native ↔ WASM**: Shared pointers to same memory
2. **Native ↔ GPU**: Device pointers via CUDA
3. **WASM ↔ GPU**: Via shared memory manager

### Safety Guarantees

1. **WASM Sandbox**: Memory isolation via Wasmtime
2. **Fuel Limits**: Prevents infinite loops (100k instructions)
3. **Capability System**: Fine-grained permissions
4. **Type Safety**: Rust's ownership model

## Scalability

- **Vertical**: Scales to all CPU cores via worker pool
- **Horizontal**: Scales across cluster nodes
- **Device**: Scales to multiple GPUs/QPUs per node
- **Plugin**: Supports thousands of concurrent WASM instances

## Future Enhancements

1. **Unified Scheduler**: Cross-tier work stealing
2. **QoS System**: Priority-based scheduling
3. **Advanced Checkpointing**: Incremental state snapshots
4. **Plugin Marketplace**: Verified WASM plugin registry
5. **Observability**: OpenTelemetry integration
