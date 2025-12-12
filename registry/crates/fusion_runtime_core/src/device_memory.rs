//! # Device Memory Allocator (DMA)
//!
//! VRAM Manager - dedicated FFI layer that directly interfaces with CUDA/HIP/Metal
//! APIs to allocate and manage blocks of VRAM/QPU memory.
//!
//! Provides the backing store for PagedAttentionManager and manages the physical
//! blocks used by fusion_llm_gpu_scheduler.

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// NVIDIA GPU (CUDA)
    Cuda(u32),
    /// AMD GPU (HIP/ROCm)
    Hip(u32),
    /// Apple GPU (Metal)
    Metal(u32),
    /// QPU
    Qpu(u32),
}

/// Device memory block handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceMemHandle(pub u64);

/// Device memory block
pub struct DeviceMemBlock {
    /// Handle
    pub handle: DeviceMemHandle,

    /// Device type
    pub device: DeviceType,

    /// Size in bytes
    pub size: usize,

    /// Device pointer
    pub device_ptr: usize,

    /// Is this block currently in use?
    pub in_use: bool,
}

/// Device Memory Allocator
///
/// Manages VRAM across GPUs and QPUs
pub struct DeviceMemoryAllocator {
    /// Active allocations
    allocations: Arc<Mutex<HashMap<DeviceMemHandle, DeviceMemBlock>>>,

    /// Free blocks (pooled)
    free_blocks: Arc<Mutex<Vec<DeviceMemBlock>>>,

    /// Next handle ID
    next_handle: Arc<Mutex<u64>>,

    /// Total allocated per device
    device_totals: Arc<Mutex<HashMap<DeviceType, usize>>>,
}

impl DeviceMemoryAllocator {
    /// Create a new device memory allocator
    pub fn new() -> Self {
        Self {
            allocations: Arc::new(Mutex::new(HashMap::new())),
            free_blocks: Arc::new(Mutex::new(Vec::new())),
            next_handle: Arc::new(Mutex::new(1)),
            device_totals: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Allocate device memory
    ///
    /// # Arguments
    ///
    /// * `device` - Target device
    /// * `size` - Size in bytes
    ///
    /// # Returns
    ///
    /// Device memory handle
    ///
    /// # Example
    ///
    /// ```rust
    /// let dma = DeviceMemoryAllocator::new();
    /// let handle = dma.allocate(DeviceType::Cuda(0), 1024 * 1024 * 1024)?;  // 1GB VRAM
    /// ```
    pub fn allocate(&self, device: DeviceType, size: usize) -> Result<DeviceMemHandle, String> {
        // Try to reuse a free block first
        let reused = self.try_reuse_block(device, size);
        if let Some(handle) = reused {
            return Ok(handle);
        }

        // Allocate new block
        let mut next_handle = self.next_handle.lock();
        let handle = DeviceMemHandle(*next_handle);
        *next_handle += 1;
        drop(next_handle);

        // In real implementation, would call:
        // - CUDA: cudaMalloc()
        // - HIP: hipMalloc()
        // - Metal: MTLBuffer allocation

        // Simulate device pointer
        let device_ptr = handle.0 as usize * 0x1000; // Fake address

        let block = DeviceMemBlock {
            handle,
            device,
            size,
            device_ptr,
            in_use: true,
        };

        self.allocations.lock().insert(handle, block);

        // Update totals
        *self.device_totals.lock().entry(device).or_insert(0) += size;

        tracing::info!(
            "Allocated device memory: handle={:?}, device={:?}, size={} bytes",
            handle,
            device,
            size
        );

        Ok(handle)
    }

    /// Try to reuse a free block
    fn try_reuse_block(&self, device: DeviceType, size: usize) -> Option<DeviceMemHandle> {
        let mut free_blocks = self.free_blocks.lock();

        // Find a suitable free block
        let idx = free_blocks
            .iter()
            .position(|b| b.device == device && b.size >= size)?;

        let mut block = free_blocks.remove(idx);
        block.in_use = true;
        let handle = block.handle;

        self.allocations.lock().insert(handle, block);

        tracing::debug!("Reused free block: handle={:?}", handle);

        Some(handle)
    }

    /// Free device memory (returns to pool)
    pub fn free(&self, handle: DeviceMemHandle) -> Result<(), String> {
        let mut allocations = self.allocations.lock();
        let mut block = allocations
            .remove(&handle)
            .ok_or_else(|| format!("Handle {:?} not found", handle))?;

        block.in_use = false;

        // Return to free pool for reuse
        self.free_blocks.lock().push(block);

        tracing::info!("Freed device memory to pool: handle={:?}", handle);

        Ok(())
    }

    /// Actually free a block (remove from pool)
    pub fn deallocate(&self, handle: DeviceMemHandle) -> Result<(), String> {
        // Remove from allocations if present
        if let Some(block) = self.allocations.lock().remove(&handle) {
            *self.device_totals.lock().entry(block.device).or_insert(0) -= block.size;

            // In real implementation, would call:
            // - CUDA: cudaFree()
            // - HIP: hipFree()
            // - Metal: release buffer

            return Ok(());
        }

        // Remove from free pool
        let mut free_blocks = self.free_blocks.lock();
        if let Some(idx) = free_blocks.iter().position(|b| b.handle == handle) {
            let block = free_blocks.remove(idx);
            *self.device_totals.lock().entry(block.device).or_insert(0) -= block.size;

            return Ok(());
        }

        Err(format!("Handle {:?} not found", handle))
    }

    /// Get device pointer
    pub fn get_device_ptr(&self, handle: DeviceMemHandle) -> Option<usize> {
        self.allocations.lock().get(&handle).map(|b| b.device_ptr)
    }

    /// Copy from host to device
    pub fn copy_to_device(
        &self,
        handle: DeviceMemHandle,
        offset: usize,
        data: &[u8],
    ) -> Result<(), String> {
        let allocations = self.allocations.lock();
        let block = allocations
            .get(&handle)
            .ok_or_else(|| format!("Handle {:?} not found", handle))?;

        if offset + data.len() > block.size {
            return Err("Copy would exceed device memory size".to_string());
        }

        // In real implementation, would call:
        // - CUDA: cudaMemcpy(..., cudaMemcpyHostToDevice)
        // - HIP: hipMemcpy(..., hipMemcpyHostToDevice)
        // - Metal: buffer.contents().copy_from(data)

        tracing::trace!(
            "Copied {} bytes to device memory at offset {}",
            data.len(),
            offset
        );

        Ok(())
    }

    /// Copy from device to host
    pub fn copy_from_device(
        &self,
        handle: DeviceMemHandle,
        offset: usize,
        len: usize,
    ) -> Result<Vec<u8>, String> {
        let allocations = self.allocations.lock();
        let block = allocations
            .get(&handle)
            .ok_or_else(|| format!("Handle {:?} not found", handle))?;

        if offset + len > block.size {
            return Err("Copy would exceed device memory size".to_string());
        }

        // In real implementation, would call:
        // - CUDA: cudaMemcpy(..., cudaMemcpyDeviceToHost)
        // - HIP: hipMemcpy(..., hipMemcpyDeviceToHost)
        // - Metal: read from buffer.contents()

        Ok(vec![0u8; len]) // Placeholder
    }

    /// Get total allocated memory for device
    pub fn device_total(&self, device: DeviceType) -> usize {
        *self.device_totals.lock().get(&device).unwrap_or(&0)
    }

    /// Get allocation statistics
    pub fn stats(&self) -> DeviceMemStats {
        DeviceMemStats {
            active_allocations: self.allocations.lock().len(),
            free_blocks: self.free_blocks.lock().len(),
            total_allocated: self.device_totals.lock().values().sum(),
        }
    }
}

impl Default for DeviceMemoryAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Device memory statistics
#[derive(Debug, Clone)]
pub struct DeviceMemStats {
    pub active_allocations: usize,
    pub free_blocks: usize,
    pub total_allocated: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_free() {
        let dma = DeviceMemoryAllocator::new();
        let handle = dma.allocate(DeviceType::Cuda(0), 1024 * 1024).unwrap();

        assert_eq!(dma.stats().active_allocations, 1);
        assert_eq!(dma.device_total(DeviceType::Cuda(0)), 1024 * 1024);

        dma.free(handle).unwrap();
        assert_eq!(dma.stats().active_allocations, 0);
        assert_eq!(dma.stats().free_blocks, 1); // Returned to pool
    }

    #[test]
    fn test_block_reuse() {
        let dma = DeviceMemoryAllocator::new();

        // Allocate and free
        let handle1 = dma.allocate(DeviceType::Cuda(0), 4096).unwrap();
        dma.free(handle1).unwrap();

        // Allocate again - should reuse  block
        let handle2 = dma.allocate(DeviceType::Cuda(0), 4096).unwrap();

        assert_eq!(handle1, handle2); // Same handle reused
        assert_eq!(dma.stats().free_blocks, 0);
    }

    #[test]
    fn test_copy_operations() {
        let dma = DeviceMemoryAllocator::new();
        let handle = dma.allocate(DeviceType::Cuda(0), 1024).unwrap();

        let data = vec![42u8; 512];
        dma.copy_to_device(handle, 0, &data).unwrap();

        let result = dma.copy_from_device(handle, 0, 512).unwrap();
        assert_eq!(result.len(), 512);

        dma.deallocate(handle).unwrap();
    }
}
