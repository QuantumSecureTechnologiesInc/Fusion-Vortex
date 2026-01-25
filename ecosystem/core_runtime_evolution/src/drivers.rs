// Status: Metal Layer
// Purpose: Hardware Abstraction Layer (HAL) for Linux io_uring and CUDA.
// Dependencies: io-uring, libc

use std::os::unix::io::RawFd;
use std::sync::Mutex;
use io_uring::{IoUring, opcode, types};
use crate::error::{FusionError, Result};

/// Trait defining a generic non-blocking driver.
pub trait Driver: Send + Sync {
    fn name(&self) -> &str;
    fn init(&self) -> Result<()>;
}

// -----------------------------------------------------------------------------
// Linux I/O Driver (Real io_uring implementation)
// -----------------------------------------------------------------------------

/// Metal driver for Linux asynchronous I/O.
///
/// This driver wraps the `io_uring` interface, allowing us to submit
/// system calls asynchronously to the kernel without the overhead of
/// standard synchronous syscalls.
pub struct IoUringDriver {
    /// The actual ring instance. Wrapped in Mutex because `submission()` requires mutable access.
    /// In a high-throughput scenario, we might use a thread-local ring to avoid locking.
    ring: Mutex<IoUring>,
    queue_depth: u32,
}

impl IoUringDriver {
    /// Initialises the io_uring with the specified queue depth.
    pub fn new(queue_depth: u32) -> Result<Self> {
        let ring = IoUring::new(queue_depth)
            .map_err(|e| FusionError::Io(e))?;
        if queue_depth == 0 {
            return Err(FusionError::ExecutorOverload);
        }
            
        Ok(Self {
            ring: Mutex::new(ring),
            queue_depth,
        })
    }

    /// Submits a read request to the ring.
    ///
    /// # Safety
    /// This function is unsafe because it passes a raw pointer (`buf`) to the kernel.
    /// The caller must ensure that `buf` remains valid until the operation completes.
    pub unsafe fn submit_read(
        &self, 
        token: u64, 
        fd: RawFd, 
        buf: *mut u8, 
        len: u32, 
        offset: u64
    ) -> Result<()> {
        let mut ring = self.ring.lock().map_err(|_| FusionError::ReactorStall("Ring lock poisoned".into()))?;
        
        // 1. Construct the Read operation.
        // Fd: The file descriptor to read from.
        // Buf: The destination buffer.
        // Len: Number of bytes to read.
        let op = opcode::Read::new(types::Fd(fd), buf, len)
            .offset(offset)
            .build()
            .user_data(token); // Attach the token so we know which task to wake later.

        // 2. Push to the Submission Queue.
        // Note: explicit unsafe block for the queue interaction if required by the crate version,
        // but opcode construction handles most safety invariants.
        unsafe {
            ring.submission()
                .push(&op)
                .map_err(|_| FusionError::ExecutorOverload)?; // Queue full
        }

        // 3. Submit to kernel.
        ring.submit()
            .map_err(|e| FusionError::Io(e))?;

        Ok(())
    }

    /// Waits for completions and returns the tokens of completed tasks.
    pub fn wait_for_completions(&self) -> Result<Vec<(u64, i32)>> {
        let mut ring = self.ring.lock().map_err(|_| FusionError::ReactorStall("Ring lock poisoned".into()))?;
        
        // Initiate the syscall to wait for at least one event.
        ring.submit_and_wait(1)
            .map_err(|e| FusionError::Io(e))?;

        let mut completions = Vec::new();
        let cq = ring.completion();

        for cqe in cq {
            // user_data is the token we passed in `submit_read`.
            // result is the return value of the syscall (e.g., bytes read).
            completions.push((cqe.user_data(), cqe.result()));
        }

        Ok(completions)
    }
}

impl Driver for IoUringDriver {
    fn name(&self) -> &str {
        "HyperRing-Linux-v6.x"
    }

    fn init(&self) -> Result<()> {
        if self.queue_depth == 0 {
            return Err(FusionError::ExecutorOverload);
        }
        // Validation: Ensure the kernel supports necessary features (e.g., IORING_FEAT_NODROP)
        let ring = self.ring.lock().unwrap();
        let params = ring.params();
        if !params.is_feature_nodrop() {
            return Err(FusionError::Io(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "Kernel too old: missing IORING_FEAT_NODROP"
            )));
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// CUDA FFI Driver (Stubbed for compilation context, ready for linking)
// -----------------------------------------------------------------------------

pub struct CudaDriver {
    device_id: i32,
}

impl CudaDriver {
    pub fn new(device_id: i32) -> Self {
        Self { device_id }
    }
    
    // In a full production build, we would use `libloading` here to load libcuda.so
    // to avoid build-time linking requirements on non-GPU dev machines.
}

impl Driver for CudaDriver {
    fn name(&self) -> &str {
        "NVIDIA-CUDA-v12"
    }
    fn init(&self) -> Result<()> {
        if self.device_id < 0 {
            return Err(FusionError::InvalidConfig("Invalid CUDA device id".into()));
        }
        Ok(())
    }
}



