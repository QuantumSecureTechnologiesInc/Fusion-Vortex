//! Task abstraction and handle

use parking_lot::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// A spawned task
pub struct Task {
    id: u64,
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

/// Handle to a spawned task
pub struct TaskHandle<T> {
    id: u64,
    result: Arc<Mutex<Option<Result<T, JoinError>>>>,
}

impl<T> TaskHandle<T> {
    pub(crate) fn new(id: u64) -> Self {
        Self {
            id,
            result: Arc::new(Mutex::new(None)),
        }
    }
}

impl<T> Future for TaskHandle<T>
where
    T: Clone,
{
    type Output = Result<T, JoinError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(result) = self.result.lock().as_ref() {
            Poll::Ready(result.clone())
        } else {
            Poll::Pending
        }
    }
}

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
