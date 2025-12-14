# Executor

The execution engine for Fusion, responsible for running tasks, managing threads, and scheduling work.

## Features
- Task scheduling
- Thread pool management
- Async runtime integration

## Usage
Used internally by `fusion_runtime_core` but can be used standalone.
```rust
use executor::ThreadPool;
```
