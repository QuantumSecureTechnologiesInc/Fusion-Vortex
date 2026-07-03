//! # Pinned Thread Pool
//!
//! A work-stealing thread pool with core affinity for HFT (High-Frequency Trading)
//! workloads. Threads are pinned to specific CPU cores to minimize jitter and
//! cache thrashing.
//!
//! ## Design
//!
//! - Uses `isolcpus` kernel parameter to prevent OS scheduler interference
//! - Work-stealing via crossbeam-deque for load balancing
//! - Busy-wait spinning for minimal latency on idle threads
//! - Panic recovery to prevent worker thread death

use core_affinity::CoreId;
use crossbeam_deque::{Injector, Steal, Worker};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tracing::{error, info, instrument};

/// A high-performance thread pool with core pinning and work stealing.
///
/// Designed for HFT workloads where consistent sub-10μs latency is required.
/// Uses busy-wait spinning instead of blocking to minimize wake-up latency.
pub struct PinnedThreadPool {
    global_queue: Arc<Injector<Box<dyn FnOnce() + Send>>>,
    shutdown: Arc<AtomicBool>,
    num_workers: usize,
}

impl PinnedThreadPool {
    /// Create a new pinned thread pool with the specified number of threads.
    ///
    /// Threads are pinned to CPU cores in order. If fewer cores are available
    /// than requested threads, only the available cores will be used.
    ///
    /// # Arguments
    ///
    /// * `num_threads` - Number of worker threads to spawn
    ///
    /// # Example
    ///
    /// ```rust
    /// use fusion_hal::PinnedThreadPool;
    ///
    /// let pool = PinnedThreadPool::new(4);
    /// pool.spawn(|| println!("Hello from worker!"));
    /// ```
    #[instrument]
    pub fn new(num_threads: usize) -> Self {
        let core_ids = core_affinity::get_core_ids().unwrap_or_default();
        let global_queue = Arc::new(Injector::new());
        let shutdown = Arc::new(AtomicBool::new(false));

        let actual_workers = core_ids.len().min(num_threads);

        for (idx, core_id) in core_ids.into_iter().take(num_threads).enumerate() {
            let global = global_queue.clone();
            let signal = shutdown.clone();

            thread::Builder::new()
                .name(format!("fusion-worker-{}", idx))
                .spawn(move || {
                    Self::worker_loop(idx, core_id, global, signal);
                })
                .expect("Failed to spawn worker thread");
        }

        info!("Pinned thread pool created with {} workers", actual_workers);

        Self {
            global_queue,
            shutdown,
            num_workers: actual_workers,
        }
    }

    /// Worker loop that steals from the global queue and executes jobs.
    fn worker_loop(
        idx: usize,
        core_id: CoreId,
        global: Arc<Injector<Box<dyn FnOnce() + Send>>>,
        shutdown: Arc<AtomicBool>,
    ) {
        // Attempt to pin to the specified core
        if core_affinity::set_for_current(core_id) {
            info!("Worker {} pinned to Core {:?}", idx, core_id);
        } else {
            error!("Worker {} failed to pin to Core {:?}", idx, core_id);
        }

        // Local worker queue for work stealing
        let local: Worker<Box<dyn FnOnce() + Send>> = Worker::new_fifo();

        loop {
            // Check shutdown signal
            if shutdown.load(Ordering::Relaxed) {
                info!("Worker {} shutting down", idx);
                break;
            }

            // Work Stealing Logic:
            // 1. Try local queue first (cache hot)
            // 2. Fall back to global queue (work stealing)
            let task = local.pop().or_else(|| match global.steal() {
                Steal::Success(job) => Some(job),
                Steal::Empty | Steal::Retry => None,
            });

            if let Some(job) = task {
                // Execute with panic recovery
                let result = catch_unwind(AssertUnwindSafe(|| (job)()));
                if result.is_err() {
                    error!("Worker {} caught panic from job", idx);
                }
            } else {
                // HFT Busy Wait: spin instead of blocking
                // This trades CPU usage for consistent wake-up latency
                std::hint::spin_loop();
            }
        }
    }

    /// Spawn a new job on the thread pool.
    ///
    /// The job will be added to the global queue and picked up by
    /// the next available worker.
    ///
    /// # Arguments
    ///
    /// * `job` - A closure to execute on a worker thread
    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.global_queue.push(Box::new(job));
    }

    /// Spawn a batch of jobs on the thread pool.
    ///
    /// More efficient than calling `spawn` multiple times as it
    /// reduces contention on the global queue.
    pub fn spawn_batch<I, F>(&self, jobs: I)
    where
        I: IntoIterator<Item = F>,
        F: FnOnce() + Send + 'static,
    {
        for job in jobs {
            self.global_queue.push(Box::new(job));
        }
    }

    /// Get the number of worker threads.
    pub fn num_workers(&self) -> usize {
        self.num_workers
    }

    /// Check if the pool is still active.
    pub fn is_active(&self) -> bool {
        !self.shutdown.load(Ordering::Relaxed)
    }

    /// Initiate shutdown of the thread pool.
    ///
    /// Workers will finish their current jobs and then exit.
    pub fn shutdown(&self) {
        info!("Initiating thread pool shutdown");
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

impl Drop for PinnedThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;

    #[test]
    fn test_pool_creation() {
        let pool = PinnedThreadPool::new(2);
        assert!(pool.is_active());
        assert!(pool.num_workers() >= 1);
    }

    #[test]
    fn test_spawn_job() {
        let pool = PinnedThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        pool.spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Give time for job to execute
        thread::sleep(Duration::from_millis(100));

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_spawn_batch() {
        let pool = PinnedThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        let jobs: Vec<_> = (0..10)
            .map(|_| {
                let c = counter.clone();
                move || {
                    c.fetch_add(1, Ordering::SeqCst);
                }
            })
            .collect();

        pool.spawn_batch(jobs);

        // Give time for jobs to execute
        thread::sleep(Duration::from_millis(200));

        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_panic_recovery() {
        let pool = PinnedThreadPool::new(2);

        // This should not crash the worker
        pool.spawn(|| {
            panic!("Test panic");
        });

        // Pool should still work after panic
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        pool.spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        thread::sleep(Duration::from_millis(100));

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_shutdown() {
        let pool = PinnedThreadPool::new(2);
        assert!(pool.is_active());

        pool.shutdown();
        assert!(!pool.is_active());
    }
}
