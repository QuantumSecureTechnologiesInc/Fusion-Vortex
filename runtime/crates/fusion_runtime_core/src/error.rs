//! # Fusion Error Types
//!
//! Comprehensive error handling for the Fusion Runtime.
//! Integrated from fusion_core.

use std::fmt;

/// Main error type for Fusion operations
#[derive(Debug, Clone)]
pub enum FusionError {
    /// Type mismatch between incompatible types
    TypeMismatch { expected: String, found: String },

    /// Shape mismatch in tensor operations
    ShapeMismatch {
        op: String,
        lhs: Vec<usize>,
        rhs: Vec<usize>,
    },

    /// Index out of bounds in tensor access
    IndexOutOfBounds {
        indices: Vec<usize>,
        shape: Vec<usize>,
    },

    /// Quantum gate arity mismatch
    GateArityMismatch {
        gate: String,
        required: usize,
        provided: usize,
    },

    /// Invalid qubit access
    InvalidQubitAccess(usize),

    /// Quantum no-cloning theorem violation
    NoCloning { qubit_id: usize },

    /// Measurement of already-measured qubit
    AlreadyMeasured { qubit_id: usize },

    /// Matrix dimension mismatch
    MatrixDimensionMismatch {
        op: String,
        lhs_shape: (usize, usize),
        rhs_shape: (usize, usize),
    },

    /// Compilation error
    CompilationError(String),

    /// Runtime error
    RuntimeError(String),

    /// I/O error
    IoError(String),

    /// Generic error with message
    Other(String),
}

impl fmt::Display for FusionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            Self::ShapeMismatch { op, lhs, rhs } => {
                write!(f, "Shape mismatch in {}: {:?} vs {:?}", op, lhs, rhs)
            }
            Self::IndexOutOfBounds { indices, shape } => {
                write!(f, "Index {:?} out of bounds for shape {:?}", indices, shape)
            }
            Self::GateArityMismatch {
                gate,
                required,
                provided,
            } => {
                write!(
                    f,
                    "Gate {} requires {} qubits, got {}",
                    gate, required, provided
                )
            }
            Self::InvalidQubitAccess(id) => {
                write!(f, "Invalid qubit access: qubit {} does not exist", id)
            }
            Self::NoCloning { qubit_id } => {
                write!(
                    f,
                    "Cannot clone quantum state: qubit {} (no-cloning theorem)",
                    qubit_id
                )
            }
            Self::AlreadyMeasured { qubit_id } => {
                write!(f, "Qubit {} has already been measured", qubit_id)
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
            Self::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
            Self::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for FusionError {}

/// Result type alias for Fusion operations
pub type FusionResult<T> = Result<T, FusionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FusionError::TypeMismatch {
            expected: "Tensor".to_string(),
            found: "Quantum".to_string(),
        };
        assert!(err.to_string().contains("Type mismatch"));
    }

    #[test]
    fn test_shape_mismatch() {
        let err = FusionError::ShapeMismatch {
            op: "matmul".to_string(),
            lhs: vec![2, 3],
            rhs: vec![4, 5],
        };
        assert!(err.to_string().contains("Shape mismatch"));
    }
}
