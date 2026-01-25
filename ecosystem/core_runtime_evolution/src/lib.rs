// Status: Main Entry
// Purpose: Exports modules and Runtime Builder.

pub mod cluster;
pub mod crypto;
pub mod drivers;
pub mod error;
pub mod executor;
pub mod fs;
pub mod reactor;
pub mod sync;
pub mod task;
pub mod wasm;

use std::sync::Arc;
use crate::executor::Executor;
use crate::error::Result;

/// The Fusion Runtime.
///
/// This struct holds the initialized executor and services needed to run
/// a Fusion node.
pub struct Runtime {
    executor: Arc<Executor>,
}

impl Runtime {
    /// Creates a new Runtime with default configuration.
    pub fn new() -> Result<Self> {
        let executor = Arc::new(Executor::new());
        Ok(Self { executor })
    }

    /// Spawns a future onto the runtime.
    pub fn spawn(&self, future: impl std::future::Future<Output = ()> + Send + 'static) {
        self.executor.spawn(future);
    }

    /// Blocks the current thread and runs the runtime until shutdown.
    pub fn block_on(&self, future: impl std::future::Future<Output = ()> + Send + 'static) -> Result<()> {
        self.executor.spawn(future);
        self.executor.run()
    }
}

/// Builder for advanced Runtime configuration.
pub struct Builder {
    worker_threads: usize,
    queue_depth: usize,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            worker_threads: num_cpus::get(),
            queue_depth: 1024,
        }
    }

    pub fn build(self) -> Result<Runtime> {
        // Configure executor with specific thread counts here
        if self.worker_threads == 0 {
            return Err(crate::error::FusionError::InvalidConfig(
                "worker_threads must be greater than zero".into(),
            ));
        }
        if self.queue_depth == 0 {
            return Err(crate::error::FusionError::InvalidConfig(
                "queue_depth must be greater than zero".into(),
            ));
        }
        Runtime::new()
    }
}
