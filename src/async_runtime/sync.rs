// src/async_runtime/sync.rs - Async Synchronization Primitives
#![allow(dead_code)]
// Implements MPSC Channel

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Shared channel state
struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    waker: Mutex<Option<Waker>>,
    closed: Mutex<bool>,
}

/// Sender half
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// Receiver half
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&self, val: T) {
        let mut queue = self.shared.queue.lock().unwrap();
        queue.push_back(val);
        // Wake receiver if waiting
        let mut waker = self.shared.waker.lock().unwrap();
        if let Some(w) = waker.take() {
            w.wake();
        }
    }
}

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        RecvFuture {
            shared: &self.shared,
        }
        .await
    }

    /// Non-blocking receive for demo purposes
    pub fn try_recv(&self) -> Option<T> {
        let mut queue = self.shared.queue.lock().unwrap();
        queue.pop_front()
    }
}

struct RecvFuture<'a, T> {
    shared: &'a Arc<Shared<T>>,
}

impl<'a, T> Future for RecvFuture<'a, T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut queue = self.shared.queue.lock().unwrap();

        if let Some(val) = queue.pop_front() {
            return Poll::Ready(Some(val));
        }

        if *self.shared.closed.lock().unwrap() {
            return Poll::Ready(None);
        }

        // Register waker
        let mut waker = self.shared.waker.lock().unwrap();
        *waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// Create a new channel
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        queue: Mutex::new(VecDeque::new()),
        waker: Mutex::new(None),
        closed: Mutex::new(false),
    });

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver { shared },
    )
}
