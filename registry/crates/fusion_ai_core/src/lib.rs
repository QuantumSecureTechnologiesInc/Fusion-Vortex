//! # Fusion AI Core
//!
//! AI/ML primitives with automatic differentiation and zero-copy tensor operations.
//!
//! This crate leverages `fusion_runtime_core` for GPU scheduling and memory management,
//! enabling true zero-copy data flow between training and inference.

use fusion_core::{FusionType, TensorDType, TensorType};

use tracing::{debug, trace};

pub mod nn;
pub mod ops;
pub mod optim;

/// Tensor builder for AI/ML workloads
pub struct Tensor {
    shape: Vec<usize>,
    dtype: TensorDType,
    device: String,
    requires_grad: bool,
}

impl Tensor {
    /// Create a tensor of zeros
    pub fn zeros(shape: impl Into<Vec<usize>>) -> Self {
        let shape = shape.into();
        debug!("Creating zero tensor with shape {:?}", shape);

        Self {
            shape,
            dtype: TensorDType::F32,
            device: "cpu".to_string(),
            requires_grad: false,
        }
    }

    /// Create a tensor of ones
    pub fn ones(shape: impl Into<Vec<usize>>) -> Self {
        let shape = shape.into();
        debug!("Creating ones tensor with shape {:?}", shape);

        Self {
            shape,
            dtype: TensorDType::F32,
            device: "cpu".to_string(),
            requires_grad: false,
        }
    }

    /// Set the device for this tensor
    pub fn device(mut self, device: impl Into<String>) -> Self {
        self.device = device.into();
        self
    }

    /// Enable gradient tracking for this tensor
    pub fn requires_grad(mut self, requires_grad: bool) -> Self {
        self.requires_grad = requires_grad;
        self
    }

    /// Matrix multiplication (zero-copy when on GPU)
    pub async fn matmul(&self, other: &Tensor) -> Tensor {
        trace!(
            "Matrix multiplication: {:?} @ {:?}",
            self.shape,
            other.shape
        );

        // In a real implementation, this would:
        // 1. Check device compatibility
        // 2. Schedule GPU kernel via fusion_runtime_core
        // 3. Perform zero-copy operation if possible
        // 4. Return new tensor referencing result

        let result_rows = self.shape[0];
        let result_cols = other.shape[1];

        Tensor {
            shape: vec![result_rows, result_cols],
            dtype: self.dtype,
            device: self.device.clone(),
            requires_grad: self.requires_grad || other.requires_grad,
        }
    }

    /// Get tensor shape
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn from_slice(_data: &[f64]) -> Self {
        // Mock creation from slice, assuming 1D for now
        Self::zeros(vec![_data.len()])
    }

    pub fn eye(n: usize) -> Self {
        // Mock identity matrix
        Self::ones(vec![n, n])
    }

    pub fn mean(&self) -> Self {
        // Mock mean, returns scalar tensor
        Self::ones(vec![1])
    }

    pub fn item<T: Default>(&self) -> T {
        // Mock item retrieval
        T::default()
    }
}

impl From<Tensor> for FusionType {
    fn from(tensor: Tensor) -> Self {
        FusionType::Tensor(TensorType {
            shape: tensor.shape,
            dtype: tensor.dtype,
            device: tensor.device,
            data_ptr: 0, // Would be allocated by memory manager
        })
    }
}

/// Automatic differentiation engine
pub struct Autodiff {
    // Computation graph would be stored here
}

impl Autodiff {
    pub fn new() -> Self {
        debug!("Initialising autodiff engine");
        Self {}
    }

    /// Compute gradients via backpropagation
    pub fn backward(&mut self, loss: &Tensor) {
        trace!("Computing gradients for loss tensor");

        // In a real implementation:
        // 1. Traverse computation graph in reverse
        // 2. Apply chain rule
        // 3. Update gradients in-place (zero-copy)
    }
}

impl Default for Autodiff {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::zeros(vec![2, 3]);
        assert_eq!(tensor.shape(), &[2, 3]);
    }

    #[tokio::test]
    async fn test_matrix_multiplication() {
        let a = Tensor::zeros(vec![2, 3]);
        let b = Tensor::zeros(vec![3, 4]);
        let c = a.matmul(&b).await;

        assert_eq!(c.shape(), &[2, 4]);
    }
}
