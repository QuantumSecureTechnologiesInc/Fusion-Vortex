//! Nebula Legacy Runtime (v2.0)
//! Focus: Synchronous execution, strict deterministic thread scheduling.
//! 
//! Used for legacy infrastructure where jitter-free execution is required.
use crate::types::*;

pub struct NebulaKernel {
    pub thread_id: FSize,
    pub is_locked: FBool,
}

impl NebulaKernel {
    pub fn new() -> Self {
        Self { thread_id: 0, is_locked: false }
    }

    /// Executes a closure strictly synchronously.
    pub fn execute_sync<F>(&self, work: F) where F: FnOnce() {
        // Nebula forces synchronous blocking execution.
        work();
    }

    /// Locks the kernel for a critical section.
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
    }
}