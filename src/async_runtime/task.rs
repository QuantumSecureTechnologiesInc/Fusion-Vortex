// src/async_runtime/task.rs - Async Task Definition

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Unique Task Identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

/// Async Task wrapper
#[allow(dead_code)]
pub struct Task {
    pub id: TaskId,
    pub future: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Task {
    #[allow(dead_code)]
    pub fn new(future: impl Future<Output = ()> + Send + 'static, id: u64) -> Self {
        Self {
            id: TaskId(id),
            future: Box::pin(future),
        }
    }

    #[allow(dead_code)]
    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}
