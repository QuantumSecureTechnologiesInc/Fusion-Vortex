use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum FusionError {
    #[error("Shape mismatch in {op}: {lhs:?} vs {rhs:?}")]
    ShapeMismatch {
        op: String,
        lhs: Vec<usize>,
        rhs: Vec<usize>,
    },

    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),

    #[error("Unknown variable: {0}")]
    UnknownVariable(String),

    #[error("Compilation error: {0}")]
    CompilationError(String),

    #[error("Fusion Error: {0}")]
    Generic(String),
}

pub type FusionResult<T> = Result<T, FusionError>;
