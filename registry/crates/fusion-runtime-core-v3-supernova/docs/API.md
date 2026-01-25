# API Reference - Fusion Runtime Core v3.0 Supernova

Complete API documentation for all public interfaces.

## Table of Contents

1. [Core Types](#core-types)
2. [Runtime & Builder](#runtime--builder)
3. [Task Spawning](#task-spawning)
4. [Device Execution](#device-execution)
5. [WASM Plugins](#wasm-plugins)
6. [Distributed Execution](#distributed-execution)
7. [Shared Memory](#shared-memory)
8. [Metrics](#metrics)
9. [Error Handling](#error-handling)

---

## Core Types

### `Device`

Represents an execution device.

```rust
pub enum Device {
    Cpu,
    Gpu(u32),  // GPU device ID
    Qpu(u32),  // QPU device ID
}
```text

### `JoinHandle<T>`

Handle to a spawned task.

```rust
pub struct JoinHandle<T> {
    // ...
}

impl<T> Future for JoinHandle<T> {
    type Output = T;
    // ...
}
```text

**Usage:**

```rust
let handle = spawn(async { 42 });
let result = handle.await; // result == 42
```text

---

## Runtime & Builder

### `Builder`

Configures and builds the runtime.

```rust
pub struct Builder {
    // ...
}

impl Builder {
    pub fn new() -> Self;
    pub fn worker_threads(self, count: usize) -> Self;
    pub fn enable_gpu(self) -> Self;
    pub fn enable_wasm(self) -> Self;
    pub fn enable_distributed(self) -> Self;
    pub fn build(self) -> Runtime;
}
```text

**Example:**

```rust
let runtime = Builder::new()
    .worker_threads(8)
    .enable_gpu()
    .enable_wasm()
    .enable_distributed()
    .build();
```text

### `Runtime`

The main runtime executor.

```rust
pub struct Runtime {
    // ...
}

impl Runtime {
    pub fn new(workers: usize) -> Self;
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}
```text

**Example:**

```rust
runtime.block_on(async {
    // Your async code
});
```text

---

## Task Spawning

### `spawn()`

Spawns an async task on the runtime.

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static;
```text

**Example:**

```rust
let handle = spawn(async {
    // Task code
    42
});

let result = handle.await;
```text

### `RuntimeHandle`

Thread-local handle for spawning tasks.

```rust
pub struct RuntimeHandle {
    // ...
}

impl RuntimeHandle {
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>;
    pub fn spawn_with_affinity<F>(&self, future: F, device: Device) -> JoinHandle<F::Output>;
    pub fn sleep(&self, duration: Duration) -> Sleep;
}
```text

**Example:**

```rust
use fusion_runtime_core_v3_supernova::executor::GLOBAL_RUNTIME;

GLOBAL_RUNTIME.with(|rt| {
    let handle = rt.borrow().as_ref().unwrap();
    handle.spawn(async { /* ... */ });
});
```text

---

## Device Execution

### `spawn_on_gpu()`

Spawns a task on a specific GPU device.

```rust
impl RuntimeHandle {
    pub fn spawn_on_gpu<F>(&self, device_id: u32, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}
```text

**Example:**

```rust
let result = runtime_handle.spawn_on_gpu(0, async {
    // GPU kernel code
    "GPU computation complete"
}).await;
```text

### `spawn_on_qpu()`

Spawns a task on a specific QPU device.

```rust
impl RuntimeHandle {
    pub fn spawn_on_qpu<F>(&self, device_id: u32, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}
```text

**Example:**

```rust
let result = runtime_handle.spawn_on_qpu(0, async {
    // Quantum circuit
    vec![0u8, 1, 0, 1]
}).await;
```text

### `gpu_kernel()`

Executes a GPU kernel directly.

```rust
impl RuntimeHandle {
    pub async fn gpu_kernel(&self, device_id: u32, duration: Duration)
        -> Result<(), FusionError>;
}
```text

### `qpu_circuit()`

Executes a quantum circuit directly.

```rust
impl RuntimeHandle {
    pub async fn qpu_circuit(&self, device_id: u32, circuit_depth: u32)
        -> Result<Vec<u8>, FusionError>;
}
```text

---

## WASM Plugins

### `PluginEngine`

WASM plugin execution engine.

```rust
pub struct PluginEngine {
    // ...
}

impl PluginEngine {
    pub fn new(runtime: RuntimeHandle) -> Result<Self>;
    pub async fn load(&self, wasm_bytes: &[u8]) -> Result<Plugin>;
    pub async fn call(&self, plugin: &Plugin, func_name: &str, args: Vec<i32>) -> Result<()>;
    pub fn shared_memory(&self) -> &Arc<SharedMemoryManager>;
}
```text

**Example:**

```rust
let engine = PluginEngine::new(runtime_handle)?;
let plugin = engine.load(wasm_bytes).await?;
engine.call(&plugin, "process", vec![]).await?;
```text

### Host Functions

Functions callable from WASM plugins:

```rust
// Logging
fn host_log(level: i32, msg_ptr: i32, msg_len: i32);

// File I/O
fn host_read_file(path_ptr: i32, path_len: i32) -> i32;
fn host_write_file(fd: i32, data_ptr: i32, data_len: i32) -> i32;

// State
fn host_get_state(key_ptr: i32, key_len: i32) -> i32;
fn host_set_state(key_ptr: i32, key_len: i32, value: i32);

// GPU/QPU
fn host_gpu_compute(device_id: i32, data_ptr: i32, data_len: i32) -> i32;
fn host_qpu_execute(device_id: i32, circuit_ptr: i32, circuit_len: i32) -> i32;

// Shared Memory
fn host_shared_memory(name_ptr: i32, name_len: i32) -> i32;
```text

**WASM Plugin Example (Rust):**

```rust
extern "C" {
    fn host_gpu_compute(device_id: i32, data_ptr: *const u8, len: i32) -> i32;
}

#[no_mangle]

pub extern "C" fn process() {
    let data = vec![1, 2, 3, 4];
    unsafe {
        host_gpu_compute(0, data.as_ptr(), data.len() as i32);
    }
}
```text

---

## Distributed Execution

### `ClusterManager`

Manages distributed cluster execution.

```rust
pub struct ClusterManager {
    // ...
}

impl ClusterManager {
    pub fn new(node_id: String, reactor: Arc<HyperRing>) -> Self;
    pub async fn join_mesh(&self, seed_node: &str);
    pub async fn spawn_on_node<F>(&self, target_node: &str, future: F)
        -> Result<JoinHandle<F::Output>>;
    pub async fn spawn_distributed<F>(&self, future: F)
        -> Result<JoinHandle<F::Output>>;
    pub async fn migrate_task(&self, task_id: u64, target_node: &str) -> Result<()>;
    pub async fn checkpoint_task(&self, task_id: u64) -> Result<Vec<u8>>;
    pub async fn restore_task(&self, checkpoint: Vec<u8>) -> Result<u64>;
    pub fn node_id(&self) -> &str;
    pub fn peer_count(&self) -> usize;
}
```text

**Example:**

```rust
let cluster = ClusterManager::new("node-1".into(), reactor);
cluster.join_mesh("seed-node").await;

// Spawn on specific node
cluster.spawn_on_node("gpu-node", async {
    // Runs on gpu-node
}).await?;

// Spawn on best available node
cluster.spawn_distributed(async {
    // Runs on least-loaded node
}).await?;

// Migrate task
cluster.migrate_task(task_id, "new-node").await?;
```text

---

## Shared Memory

### `SharedTensor`

Zero-copy shared tensor.

```rust
pub struct SharedTensor {
    // ...
}

impl SharedTensor {
    pub fn new(shape: &[usize]) -> Result<Self>;
    pub fn write_native<F>(&self, f: F) -> Result<()>
    where F: FnOnce(&mut [f32]);
    pub fn read_native<F, T>(&self, f: F) -> Result<T>
    where F: FnOnce(&[f32]) -> T;
    pub fn as_wasm_ptr(&self) -> *const f32;
    pub fn as_wasm_mut_ptr(&self) -> *mut f32;
    pub fn device_ptr(&self) -> Result<*mut c_void>;  // GPU only
    pub fn allocate_on_gpu(&mut self, device_id: u32) -> Result<()>;  // GPU only
    pub fn copy_to_gpu(&self) -> Result<()>;  // GPU only
    pub fn copy_from_gpu(&self) -> Result<()>;  // GPU only
    pub fn shape(&self) -> &[usize];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```text

**Example:**

```rust
let tensor = SharedTensor::new(&[1024, 1024])?;

// Native access
tensor.write_native(|data| {
    data[0] = 42.0;
})?;

let value = tensor.read_native(|data| data[0])?;

// WASM access (zero-copy)
let wasm_ptr = tensor.as_wasm_ptr();

// GPU access (zero-copy)
tensor.allocate_on_gpu(0)?;
let device_ptr = tensor.device_ptr()?;
```text

### `SharedMemoryManager`

Manages shared memory regions.

```rust
pub struct SharedMemoryManager {
    // ...
}

impl SharedMemoryManager {
    pub fn new() -> Self;
    pub fn create_tensor(&self, shape: &[usize]) -> Result<Arc<SharedTensor>>;
    pub fn tensor_count(&self) -> usize;
}
```text

---

## Metrics

### `RuntimeMetrics`

Comprehensive runtime metrics.

```rust
pub struct RuntimeMetrics {
    pub native_tasks_spawned: AtomicU64,
    pub native_tasks_completed: AtomicU64,
    pub gpu_kernel_launches: AtomicU64,
    pub qpu_submissions: AtomicU64,
    pub plugins_loaded: AtomicU64,
    pub plugins_executed: AtomicU64,
    pub plugin_execution_time_ms: AtomicU64,
    pub remote_tasks_spawned: AtomicU64,
    pub task_migrations: AtomicU64,
    pub zero_copy_transfers: AtomicU64,
    pub shared_memory_allocations: AtomicU64,
    pub file_operations: AtomicU64,
    pub network_operations: AtomicU64,
}

impl RuntimeMetrics {
    pub fn new() -> Self;
    pub fn snapshot(&self) -> MetricsSnapshot;
    pub fn print_summary(&self);
    // ... increment methods ...
}
```text

**Example:**

```rust
let metrics = runtime.metrics();
metrics.increment_native_tasks_spawned();

let snapshot = metrics.snapshot();
println!("Tasks spawned: {}", snapshot.native_tasks_spawned);

metrics.print_summary();
```text

---

## Error Handling

### `FusionError`

Comprehensive error type.

```rust
pub enum FusionError {
    ReactorError(String),
    TaskFailure,
    Io(std::io::Error),
    WasmTrap(String),
    ClusterError(String),
    DeviceError(u32, String),
    Timeout,
}

pub type Result<T> = std::result::Result<T, FusionError>;
```text

**Example:**

```rust
match runtime_handle.gpu_kernel(0, duration).await {
    Ok(()) => println!("Success"),
    Err(FusionError::DeviceError(id, msg)) => {
        eprintln!("GPU {} error: {}", id, msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```text

---

## Complete Example

```rust
use fusion_runtime_core_v3_supernova::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build runtime
    let runtime = Builder::new()
        .worker_threads(4)
        .enable_gpu()
        .enable_wasm()
        .enable_distributed()
        .build();

    runtime.block_on(async {
        // 1. Native tasks
        let handle = spawn(async { 42 });
        let result = handle.await;

        // 2. GPU execution
        let gpu_handle = executor::GLOBAL_RUNTIME.with(|rt| {
            rt.borrow().as_ref().unwrap()
                .spawn_on_gpu(0, async { "GPU done" })
        });
        let gpu_result = gpu_handle.await;

        // 3. WASM plugin
        let engine = wasm::PluginEngine::new(/* handle */)?;
        let plugin = engine.load(wasm_bytes).await?;
        engine.call(&plugin, "process", vec![]).await?;

        // 4. Distributed
        let cluster = cluster::ClusterManager::new("node-1".into(), /* reactor */);
        cluster.join_mesh("seed").await;
        cluster.spawn_distributed(async { /* ... */ }).await?;

        // 5. Shared memory
        let tensor = SharedTensor::new(&[1024, 1024])?;
        tensor.write_native(|data| data[0] = 42.0)?;

        Ok::<(), FusionError>(())
    })?;

    Ok(())
}
```text

---

For more examples, see the `examples/` directory in the repository.