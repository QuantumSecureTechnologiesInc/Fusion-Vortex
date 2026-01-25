// Status: Execution Layer
// Purpose: Thread pool and Task Injector.

use std::sync::Arc;
use std::thread;
use crossbeam::channel::{unbounded, Receiver, Sender};
use crate::task::{Task, TaskId};
use crate::reactor::Reactor;
use crate::error::Result;

/// The Fusion Executor.
///
/// Manages a pool of worker threads that consume tasks from a global run queue.
/// It integrates tightly with the Reactor to poll for I/O when the queue is empty.
pub struct Executor {
    task_queue_tx: Sender<Arc<Task>>,
    task_queue_rx: Receiver<Arc<Task>>,
    reactor: Arc<Reactor>,
}

impl Executor {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self {
            task_queue_tx: tx,
            task_queue_rx: rx,
            reactor: Reactor::new(),
        }
    }

    /// Spawns a future onto the executor.
    pub fn spawn(&self, future: impl std::future::Future<Output = ()> + Send + 'static) {
        // Generate a random ID (mocked here).
        let id = TaskId(0); 
        let task = Task::spawn(id, future, self.task_queue_tx.clone());
        
        // Initial inject into the queue.
        self.task_queue_tx.send(task).unwrap();
    }

    /// Runs the executor on the current thread (blocking).
    ///
    /// In a real implementation, this would spin up a thread pool.
    pub fn run(&self) -> Result<()> {
        // Main Loop
        loop {
            // 1. Try to get a task from the global queue.
            if let Ok(task) = self.task_queue_rx.try_recv() {
                // 2. Run the task.
                task.poll();
            } else {
                // 3. If no tasks, step the Reactor to process I/O.
                // This prevents busy-waiting.
                self.reactor.tick()?;
                
                // Sleep briefly to prevent 100% CPU in this loop (mock backoff).
                thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }
    
    /// Helper to get a spawner handle.
    pub fn spawner(&self) -> Spawner {
        Spawner { tx: self.task_queue_tx.clone() }
    }
}

#[derive(Clone)]
pub struct Spawner {
    tx: Sender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl std::future::Future<Output = ()> + Send + 'static) {
        let id = TaskId(0);
        let task = Task::spawn(id, future, self.tx.clone());
        self.tx.send(task).unwrap();
    }
}