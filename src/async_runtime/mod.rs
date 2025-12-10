// src/async_runtime/mod.rs - Async/Await Runtime for Fusion
#![allow(dead_code)]
// Provides Task scheduler, Executor, and async I/O primitives

pub mod executor;
pub mod future;
pub mod sync;
pub mod task;

/// Re-export channels for convenience
pub use sync as channels;

/// Runtime configuration
pub struct RuntimeConfig {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Stack size for tasks
    pub stack_size: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: 4,
            stack_size: 2 * 1024 * 1024, // 2MB
        }
    }
}
