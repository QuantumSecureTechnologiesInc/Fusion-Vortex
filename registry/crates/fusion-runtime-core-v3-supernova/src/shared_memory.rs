// src/shared_memory.rs
// Zero-copy shared memory system

use crate::error::{FusionError, Result};
use std::sync::{Arc, RwLock};

/// Shared tensor that can be accessed from native code, WASM, and GPU
pub struct SharedTensor {
    data: Arc<RwLock<Vec<f32>>>,
    shape: Vec<usize>,
    #[cfg(feature = "gpu")]
    device_ptr: Option<*mut std::ffi::c_void>,
}

unsafe impl Send for SharedTensor {}
unsafe impl Sync for SharedTensor {}

impl SharedTensor {
    /// Create a new shared tensor with the given shape
    pub fn new(shape: &[usize]) -> Result<Self> {
        let total_elements: usize = shape.iter().product();
        let data = vec![0.0f32; total_elements];

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
            shape: shape.to_vec(),
            #[cfg(feature = "gpu")]
            device_ptr: None,
        })
    }

    /// Write data from native code
    pub fn write_native<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut [f32]),
    {
        let mut data = self.data.write().map_err(|_| FusionError::Timeout)?;
        f(&mut data);
        Ok(())
    }

    /// Read data from native code
    pub fn read_native<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&[f32]) -> T,
    {
        let data = self.data.read().map_err(|_| FusionError::Timeout)?;
        Ok(f(&data))
    }

    /// Get pointer for WASM access (zero-copy)
    pub fn as_wasm_ptr(&self) -> *const f32 {
        let data = self.data.read().unwrap();
        data.as_ptr()
    }

    /// Get mutable pointer for WASM access
    pub fn as_wasm_mut_ptr(&self) -> *mut f32 {
        let mut data = self.data.write().unwrap();
        data.as_mut_ptr()
    }

    /// Get device pointer for GPU access (zero-copy)
    #[cfg(feature = "gpu")]
    pub fn device_ptr(&self) -> Result<*mut std::ffi::c_void> {
        if let Some(ptr) = self.device_ptr {
            Ok(ptr)
        } else {
            Err(FusionError::DeviceError(
                0,
                "Tensor not allocated on GPU".into(),
            ))
        }
    }

    /// Allocate tensor on GPU device
    #[cfg(feature = "gpu")]
    pub fn allocate_on_gpu(&mut self, device_id: u32) -> Result<()> {
        log::info!("Allocating tensor on GPU device {}", device_id);
        // In production, this would call cudaMalloc
        // For now, we just mark it as allocated
        self.device_ptr = Some(std::ptr::null_mut());
        Ok(())
    }

    /// Copy data to GPU
    #[cfg(feature = "gpu")]
    pub fn copy_to_gpu(&self) -> Result<()> {
        log::debug!("Copying tensor data to GPU");
        // In production: cudaMemcpy(device_ptr, host_data, size, cudaMemcpyHostToDevice)
        Ok(())
    }

    /// Copy data from GPU
    #[cfg(feature = "gpu")]
    pub fn copy_from_gpu(&self) -> Result<()> {
        log::debug!("Copying tensor data from GPU");
        // In production: cudaMemcpy(host_data, device_ptr, size, cudaMemcpyDeviceToHost)
        Ok(())
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Manager for shared memory regions
pub struct SharedMemoryManager {
    tensors: RwLock<Vec<Arc<SharedTensor>>>,
}

impl SharedMemoryManager {
    pub fn new() -> Self {
        Self {
            tensors: RwLock::new(Vec::new()),
        }
    }

    pub fn create_tensor(&self, shape: &[usize]) -> Result<Arc<SharedTensor>> {
        let tensor = Arc::new(SharedTensor::new(shape)?);
        self.tensors.write().unwrap().push(tensor.clone());
        Ok(tensor)
    }

    pub fn tensor_count(&self) -> usize {
        self.tensors.read().unwrap().len()
    }
}

impl Default for SharedMemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
