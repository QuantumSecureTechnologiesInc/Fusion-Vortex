// src/async_runtime/executor.rs - Task Executor
#![allow(unsafe_code)]

use super::task::Task;
use std::collections::VecDeque;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Simple single-threaded executor for demonstration
#[allow(dead_code)]
pub struct Executor {
    tasks: VecDeque<Task>,
}

impl Executor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    /// Spawn a task onto the executor
    #[allow(dead_code)]
    pub fn spawn(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    /// Run all tasks to completion
    #[allow(dead_code)]
    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);

            match task.poll(&mut cx) {
                Poll::Ready(_) => {
                    // Task complete
                }
                Poll::Pending => {
                    // Task pending, push back (in real executor, only on wake)
                    self.tasks.push_back(task);
                }
            }
        }
    }
}

// Dummy Waker implementation for MVP
#[allow(dead_code)]
fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), vtable)
}

#[allow(dead_code)]
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
