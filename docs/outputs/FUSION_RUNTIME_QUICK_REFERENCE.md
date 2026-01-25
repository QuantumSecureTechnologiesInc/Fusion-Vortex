# Fusion Runtime Core - Quick Reference

**Your all-in-one runtime for async I/O + quantum/GPU compute**

---

## Installation

```toml
[dependencies]
fusion_runtime_core = { path = "runtime/crates/fusion_runtime_core" }
```text

---

## Quick Start

### Basic Async Server

```rust
use fusion_runtime_core::{Runtime, net::TcpListener};

fn main() -> std::io::Result<()> {
    let rt = Runtime::new();

    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);

        loop {
            let (socket, addr) = listener.accept().await?;
            rt.spawn(handle_client(socket, addr));
        }
    })
}

async fn handle_client(socket: TcpStream, addr: SocketAddr) {
    // Handle connection
}
```text

---

## Feature Matrix

| Use Case        | Example                       | Replaces      |
| --------------- | ----------------------------- | ------------- |
| **Web Server**  | `TcpListener::bind()`         | tokio::net    |
| **HTTP Client** | `TcpStream::connect()`        | tokio::net    |
| **Timers**      | `time::sleep()`               | tokio::time   |
| **Quantum**     | `rt.submit_quantum_circuit()` | N/A (unique!) |
| **GPU**         | `rt.device_memory()`          | External libs |
| **Tensors**     | `rt.fusion_core()`            | NumPy/PyTorch |

---

## Common Patterns

### Pattern 1: Simple Async Function

```rust

#[fusion::main]  // Coming soon - or use rt.block_on()

async fn main() {
    println!("Hello async world!");
    time::sleep(Duration::from_secs(1)).await;
}
```text

### Pattern 2: Spawn Tasks

```rust
let handle = rt.spawn(async {
    // Do async work
    42
});

let result = handle.await?;
```text

### Pattern 3: Blocking Operations

```rust
let result = rt.spawn_blocking(|| {
    // CPU-intensive or blocking operation
    expensive_computation()
}).await?;
```text

### Pattern 4: Timeouts

```rust
use fusion_runtime_core::time::timeout;

match timeout(Duration::from_secs(5), slow_operation()).await {
    Ok(result) => println!("Got: {:?}", result),
    Err(_) => println!("Timed out!"),
}
```text

### Pattern 5: Intervals

```rust
let mut interval = time::interval(Duration::from_secs(1));

loop {
    interval.tick().await;
    println!("Tick!");
}
```text

### Pattern 6: Network Echo Server

```rust
let listener = TcpListener::bind("0.0.0.0:8080").await?;

loop {
    let (mut socket, _) = listener.accept().await?;

    rt.spawn(async move {
        let mut buf = vec![0u8; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => break,  // Connection closed
                Ok(n) => {
                    socket.write_all(&buf[..n]).await?;
                }
                Err(e) => break,
            }
        }
    });
}
```text

### Pattern 7: Hybrid Quantum + Network

```rust
let listener = TcpListener::bind("0.0.0.0:9000").await?;

loop {
    let (mut socket, _) = listener.accept().await?;

    rt.spawn(async move {
        // Read quantum circuit from network
        let circuit = read_circuit(&mut socket).await?;

        // Execute on QPU
        let result = rt.submit_quantum_circuit(circuit).await?;

        // Send result back
        write_result(&mut socket, result).await?;
    });
}
```text

---

## API Cheat Sheet

### Runtime

```rust
// Create runtime
let rt = Runtime::new();
let rt = Runtime::builder().enable_all().build();

// Execute futures
rt.block_on(future);
let handle = rt.spawn(future);
let handle = rt.spawn_blocking(|| { /* blocking */ });

// Shutdown
rt.shutdown_timeout(Duration::from_secs(5));
```text

### Networking

```rust
// TCP
let listener = TcpListener::bind("0.0.0.0:8080").await?;
let (stream, addr) = listener.accept().await?;
let stream = TcpStream::connect("example.com:80").await?;

// UDP
let socket = UdpSocket::bind("0.0.0.0:8080").await?;
socket.send_to(&data, addr).await?;
let (len, addr) = socket.recv_from(&mut buf).await?;
```text

### Time

```rust
// Sleep
time::sleep(Duration::from_secs(1)).await;
time::sleep_until(deadline).await;

// Intervals
let mut interval = time::interval(Duration::from_millis(100));
interval.tick().await;

// Timeouts
time::timeout(Duration::from_secs(5), future).await?;
```text

### Quantum (Unique to Fusion!)

```rust
// Submit quantum circuits
let result = rt.submit_quantum_circuit(circuit).await?;

// Access quantum registry
let registry = rt.quantum_registry();

// QPU operations
let sequencer = rt.qpu_sequencer();
```text

### GPU (Unique to Fusion!)

```rust
// Access GPU memory
let device_mem = rt.device_memory();

// Hardware abstraction layer
let hal = rt.hal();

// Check GPU availability
if rt.config().enable_gpu {
    // Use GPU
}
```text

---

## Configuration

```rust
let rt = Runtime::builder()
    // Async I/O settings
    .worker_threads(16)           // Number of async worker threads
    .max_blocking_threads(512)    // Max blocking thread pool size
    .thread_stack_size(4_194_304) // 4MB stack per thread
    .event_interval(Duration::from_micros(100))

    // Quantum/GPU settings
    .enable_gpu()                 // Enable GPU backend
    .enable_qpu()                 // Enable quantum processing
    .gpu_backend(GpuBackend::Cuda)
    .qos_mode(QoSMode::LowLatency)
    .memory_pool_size(2_147_483_648)  // 2GB

    // Enable everything
    .enable_all()

    .build();
```text

---

## Error Handling

```rust
// Use ? operator
async fn my_function() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("example.com:80").await?;
    stream.write_all(b"Hello").await?;
    Ok(())
}

// Or match
match TcpStream::connect("example.com:80").await {
    Ok(stream) => { /* use stream */ }
    Err(e) => eprintln!("Connection failed: {}", e),
}
```text

---

## Performance Tips

1. **Use spawn for I/O, spawn_blocking for CPU**

   ```rust
   rt.spawn(async_io_task());           // Good
   rt.spawn_blocking(|| cpu_task());     // Good
   rt.spawn(async { cpu_task() });       // Bad - blocks async thread
```text

2. **Buffer your reads/writes**

   ```rust
   let mut buf = vec![0u8; 8192];  // Larger buffer = fewer syscalls
```text

3. **Reuse buffers**

   ```rust
   let mut buf = Vec::with_capacity(1024);
   loop {
       buf.clear();
       socket.read_buf(&mut buf).await?;
   }
```text

4. **Use timeouts to prevent hangs**

   ```rust
   timeout(Duration::from_secs(30), operation()).await??;
```text

---

## Common Gotchas

### ❌ Don't do this:

```rust
rt.block_on(async {
    rt.block_on(async { /* nested block_on */ });  // Will deadlock!
});
```text

### ✅ Do this instead:

```rust
rt.block_on(async {
    rt.spawn(async { /* concurrent task */ }).await;  // OK
});
```text

---

### ❌ Don't do this:

```rust
rt.spawn(async {
    std::thread::sleep(Duration::from_secs(1));  // Blocks async thread!
});
```text

### ✅ Do this instead:

```rust
rt.spawn(async {
    time::sleep(Duration::from_secs(1)).await;  // Async-friendly
});
```text

---

## Migration from Tokio

| Tokio                            | Fusion                                  | Notes                     |
| -------------------------------- | --------------------------------------- | ------------------------- |
| `tokio::runtime::Runtime::new()` | `Runtime::new()`                        | ✅ Same                    |
| `rt.block_on(future)`            | `rt.block_on(future)`                   | ✅ Same                    |
| `tokio::spawn(future)`           | `rt.spawn(future)`                      | ⚠️ Needs runtime reference |
| `tokio::net::TcpListener`        | `fusion_runtime_core::net::TcpListener` | ✅ Same API                |
| `tokio::time::sleep`             | `fusion_runtime_core::time::sleep`      | ✅ Same API                |
| `tokio::task::spawn_blocking`    | `rt.spawn_blocking`                     | ⚠️ Needs runtime reference |

---

## Examples Repository

Full examples available at: `examples/fusion_runtime/`

- `echo_server.rs` - TCP echo server
- `http_client.rs` - Simple HTTP client
- `quantum_server.rs` - Quantum circuit server
- `hybrid_ml.rs` - ML training with quantum optimization

---

## Support

- **Documentation**: [Full API docs](https://docs.fusion-lang.org/runtime)
- **Issues**: [GitHub Issues](https://github.com/fusion-lang/issues)
- **Community**: [Discord](https://discord.gg/fusion)

---

**Fusion Runtime Core - One runtime to rule them all!** 🚀