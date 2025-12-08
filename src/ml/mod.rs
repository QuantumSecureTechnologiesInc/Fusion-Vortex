// src/ml/mod.rs - Machine Learning & GPU Acceleration Library
#![allow(dead_code)]
// Provides tensor operations, neural network layers, and GPU support

pub mod autodiff;
pub mod gpu;
pub mod nn;
pub mod tensor;

/// ML Error types
#[derive(Debug, Clone)]
pub enum MLError {
    /// Dimension mismatch in tensor operation
    DimensionMismatch {
        expected: Vec<usize>,
        actual: Vec<usize>,
    },
    /// Invalid shape for operation
    InvalidShape(String),
    /// GPU backend error
    GPUError(String),
    /// Device memory allocation failed
    OutOfMemory,
}

impl std::fmt::Display for MLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MLError::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {:?}, got {:?}",
                    expected, actual
                )
            }
            MLError::InvalidShape(msg) => write!(f, "Invalid shape: {}", msg),
            MLError::GPUError(msg) => write!(f, "GPU error: {}", msg),
            MLError::OutOfMemory => write!(f, "Out of device memory"),
        }
    }
}

impl std::error::Error for MLError {}

/// Device type for tensor storage
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    /// CPU (Host)
    CPU,
    /// CUDA GPU
    CUDA(usize),
    /// OpenCL Device
    OpenCL(usize),
    /// Metal Device (macOS)
    Metal(usize),
}

impl Default for Device {
    fn default() -> Self {
        Device::CPU
    }
}
