//! Error types for tensor operations

use std::fmt;

#[derive(Debug, Clone)]
pub enum TensorError {
    ShapeMismatch {
        op: String,
        lhs: Vec<usize>,
        rhs: Vec<usize>,
    },
    IndexOutOfBounds {
        indices: Vec<usize>,
        shape: Vec<usize>,
    },
    MatrixDimensionMismatch {
        op: String,
        lhs_shape: (usize, usize),
        rhs_shape: (usize, usize),
    },
}

impl fmt::Display for TensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShapeMismatch { op, lhs, rhs } => {
                write!(f, "Shape mismatch in {}: {:?} vs {:?}", op, lhs, rhs)
            }
            Self::IndexOutOfBounds { indices, shape } => {
                write!(f, "Index {:?} out of bounds for shape {:?}", indices, shape)
            }
            Self::MatrixDimensionMismatch {
                op,
                lhs_shape,
                rhs_shape,
            } => {
                write!(
                    f,
                    "Matrix dimension mismatch in {}: {:?} vs {:?}",
                    op, lhs_shape, rhs_shape
                )
            }
        }
    }
}

impl std::error::Error for TensorError {}

pub type TensorResult<T> = Result<T, TensorError>;
