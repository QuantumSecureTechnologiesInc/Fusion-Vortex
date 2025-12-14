# Dynamic Batching

A utility crate for dynamic request batching, primarily used for optimizing inference workloads.

## Features
- Configurable batch size and timeout
- Async-aware batching logic
- Backpressure handling

## Usage
```rust
use dynamic_batch::Batcher;

let batcher = Batcher::new(process_fn).max_size(8).timeout_ms(5);
```
