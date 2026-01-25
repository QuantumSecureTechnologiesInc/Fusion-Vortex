// Status: Foundation Layer
// Purpose: Async primitives compatible with the HyperRing.

use std::cell::UnsafeCell;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};
use std::future::Future;
use std::pin::Pin;

/// An asynchronous Mutex designed for the Fusion kernel.
///
/// Unlike std::sync::Mutex, waiting for this lock does not block the thread;
/// it suspends the task until the lock is available.
pub struct AsyncMutex<T> {
    state: AtomicBool, // false = unlocked, true = locked
    data: UnsafeCell<T>,
    waiters: Mutex<VecDeque<Waker>>,
}

// Safety: We ensure exclusive access via the atomic state and waker queue.
unsafe impl<T: Send> Send for AsyncMutex<T> {}
unsafe impl<T: Send> Sync for AsyncMutex<T> {}

impl<T> AsyncMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            waiters: Mutex::new(VecDeque::new()),
        }
    }

    pub fn lock(&self) -> AsyncMutexLockFuture<'_, T> {
        AsyncMutexLockFuture { mutex: self }
    }
}

pub struct AsyncMutexLockFuture<'a, T> {
    mutex: &'a AsyncMutex<T>,
}

impl<'a, T> Future for AsyncMutexLockFuture<'a, T> {
    type Output = AsyncMutexGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 1. Try to acquire the lock using Compare-And-Swap.
        // If current is false (unlocked), set to true (locked).
        if self
            .mutex
            .state
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            return Poll::Ready(AsyncMutexGuard { mutex: self.mutex });
        }

        // 2. Failed to acquire. Register waker.
        let mut waiters = self.mutex.waiters.lock().unwrap();
        
        // Simple optimisation: check one last time before parking to avoid race.
        if self.mutex.state.load(Ordering::Acquire) == false {
             drop(waiters); // Release internal lock
             cx.waker().wake_by_ref();
             return Poll::Pending;
        }

        waiters.push_back(cx.waker().clone());
        Poll::Pending
    }
}

/// RAII Guard for the AsyncMutex.
pub struct AsyncMutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
}

impl<'a, T> Deref for AsyncMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for AsyncMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for AsyncMutexGuard<'a, T> {
    fn drop(&mut self) {
        // 1. Release the lock.
        self.mutex.state.store(false, Ordering::Release);
        
        // 2. Wake the next waiting task (FIFO).
        let mut waiters = self.mutex.waiters.lock().unwrap();
        if let Some(waker) = waiters.pop_front() {
            waker.wake();
        }
    }
}