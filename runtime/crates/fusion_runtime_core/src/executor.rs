//! Task executor implementation

use crate::config::RuntimeConfig;
use crate::task::{Task, TaskHandle};
use crossbeam::channel;
use fusion_runtime_scheduler::{Scheduler, TaskPriority};
use parking_lot::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Multi-threaded task executor
pub struct Executor {
    scheduler: Arc<Scheduler>,
    workers: Vec<Worker>,
    shutdown: Arc<Mutex<bool>>,
}

impl Executor {
    pub fn new(scheduler: Arc<Scheduler>, config: &RuntimeConfig) -> Self {
        let shutdown = Arc::new(Mutex::new(false));
        let workers = (0..config.worker_threads)
            .map(|id| Worker::new(id, scheduler.clone(), shutdown.clone()))
            .collect();

        Self {
            scheduler,
            workers,
            shutdown,
        }
    }

    pub fn spawn<F>(&self, future: F, priority: TaskPriority) -> TaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.scheduler.spawn_task(future, priority)
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        // Simple block_on implementation
        // In production, this would integrate with the scheduler
        futures::executor::block_on(future)
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        *self.shutdown.lock() = true;
        // Wait for workers to finish
    }
}

struct Worker {
    id: usize,
    _handle: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, scheduler: Arc<Scheduler>, shutdown: Arc<Mutex<bool>>) -> Self {
        let handle = std::thread::Builder::new()
            .name(format!("fusion-worker-{}", id))
            .spawn(move || {
                while !*shutdown.lock() {
                    // Worker loop
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            })
            .expect("Failed to spawn worker thread");

        Self {
            id,
            _handle: handle,
        }
    }
}
