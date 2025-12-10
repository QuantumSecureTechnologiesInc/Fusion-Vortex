//! # Fusion Tensor Core
//!
//! High-performance N-dimensional tensor implementation with compile-time rank checking.
//! Integrated from fusion_core.

mod error;
mod ops;
mod tensor;

pub use error::{TensorError, TensorResult};
pub use fusion_traits::{DataType, Numeric};
pub use ops::TensorOps;
pub use tensor::{Matrix, Scalar, Tensor, Vector};
