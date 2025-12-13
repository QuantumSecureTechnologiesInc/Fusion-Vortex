//! Task abstraction and handle

// use parking_lot::Mutex; // Unused
use std::future::Future;
use std::pin::Pin;
// use std::sync::Arc; // Unused
// use std::task::{Context, Poll}; // Unused

/// A spawned task
#[allow(dead_code)]
pub struct Task {
    id: u64,
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

/// Re-export TaskHandle from scheduler to ensure compatibility
pub use fusion_runtime_scheduler::TaskHandle;

/// Error from joining a task
#[derive(Debug, Clone)]
pub enum JoinError {
    Cancelled,
    Panic,
}

impl std::fmt::Display for JoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinError::Cancelled => write!(f, "task was cancelled"),
            JoinError::Panic => write!(f, "task panicked"),
        }
    }
}

impl std::error::Error for JoinError {}
