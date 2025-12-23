//! # Fusion AI Core
//!
//! AI/ML primitives with automatic differentiation and zero-copy tensor operations.
//!
//! This crate leverages `fusion_runtime_core` for GPU scheduling and memory management,
//! enabling true zero-copy data flow between training and inference.

use fusion_core::{FusionType, TensorDType, TensorType};
use ndarray::{ArrayD, IxDyn};
use thiserror::Error;
use tracing::{debug, trace};

#[derive(Error, Debug)]
pub enum TensorError {
    #[error("Dimension mismatch: {lhs:?} @ {rhs:?}")]
    DimensionMismatch { lhs: Vec<usize>, rhs: Vec<usize> },
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("Invalid tensor shape: {0:?}")]
    InvalidShape(Vec<usize>),
}

pub type TensorResult<T> = Result<T, TensorError>;

pub use nn::Linear;
pub use nn::Module as Layer;
pub type Variable = Tensor;

pub mod nn;
pub mod ops;
pub mod optim;

/// Tensor builder for AI/ML workloads
#[derive(Debug, Clone)]
pub struct Tensor {
    pub data: ArrayD<f32>, // Using f32 as default for ML
    pub device: String,
    pub requires_grad: bool,
}

impl Tensor {
    /// Create a tensor of zeros
    pub fn zeros(shape: impl Into<Vec<usize>>) -> Self {
        let shape = shape.into();
        debug!("Creating zero tensor with shape {:?}", shape);

        let data = ArrayD::zeros(IxDyn(&shape));

        Self {
            data,
            device: "cpu".to_string(),
            requires_grad: false,
        }
    }

    /// Create a tensor of ones
    pub fn ones(shape: impl Into<Vec<usize>>) -> Self {
        let shape = shape.into();
        debug!("Creating ones tensor with shape {:?}", shape);

        let data = ArrayD::ones(IxDyn(&shape));

        Self {
            data,
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
    pub async fn matmul(&self, other: &Tensor) -> TensorResult<Tensor> {
        trace!(
            "Matrix multiplication: {:?} @ {:?}",
            self.shape(),
            other.shape()
        );

        let self_shape = self.shape();
        let other_shape = other.shape();

        // Validate dimensions
        if self_shape.len() != 2 || other_shape.len() != 2 {
            return Err(TensorError::UnsupportedOperation(format!(
                "Matmul only supports 2D tensors, got {:?} and {:?}",
                self_shape, other_shape
            )));
        }

        let _rows = self_shape[0];
        let _cols = other_shape[1];
        let common = self_shape[1];

        if common != other_shape[0] {
            return Err(TensorError::DimensionMismatch {
                lhs: self_shape.to_vec(),
                rhs: other_shape.to_vec(),
            });
        }

        // Convert to 2D view for ndarray dot
        let a = self
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|_| TensorError::InvalidShape(self_shape.to_vec()))?;
        let b = other
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|_| TensorError::InvalidShape(other_shape.to_vec()))?;

        let result = a.dot(&b);

        Ok(Tensor {
            data: result.into_dyn(),
            device: self.device.clone(),
            requires_grad: self.requires_grad || other.requires_grad,
        })
    }

    /// Get tensor shape
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    pub fn from_slice(data: &[f64]) -> Self {
        let data_f32: Vec<f32> = data.iter().map(|&x| x as f32).collect();
        let arr = ArrayD::from_shape_vec(IxDyn(&[data.len()]), data_f32).unwrap();

        Self {
            data: arr,
            device: "cpu".to_string(),
            requires_grad: false,
        }
    }

    pub fn eye(n: usize) -> Self {
        let arr = ArrayD::from_shape_fn(IxDyn(&[n, n]), |d| if d[0] == d[1] { 1.0 } else { 0.0 });

        Self {
            data: arr,
            device: "cpu".to_string(),
            requires_grad: false,
        }
    }

    pub fn mean(&self) -> Self {
        let mean_val = self.data.mean().unwrap_or(0.0);
        let arr = ArrayD::from_elem(IxDyn(&[1]), mean_val);

        Self {
            data: arr,
            device: self.device.clone(),
            requires_grad: false,
        }
    }

    pub fn item<T>(&self) -> T
    where
        T: From<f32>,
    {
        // Assuming single item tensor
        let val = self.data.first().cloned().unwrap_or(0.0);
        T::from(val)
    }

    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            device: self.device.clone(),
            requires_grad: self.requires_grad,
        }
    }
}

impl From<Tensor> for FusionType {
    fn from(tensor: Tensor) -> Self {
        FusionType::Tensor(TensorType {
            shape: tensor.shape().to_vec(),
            dtype: TensorDType::F32,
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
    pub fn backward(&mut self, _loss: &Tensor) {
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
        let a = Tensor::eye(2); // 2x2 identity
        let b = Tensor::ones(vec![2, 2]); // 2x2 ones
        let c = a.matmul(&b).await;

        assert_eq!(c.shape(), &[2, 2]);
        // Result should be ones
        assert_eq!(c.data.sum(), 4.0);
    }
}
