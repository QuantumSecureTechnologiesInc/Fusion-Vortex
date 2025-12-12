use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionError {
    ShapeMismatch {
        op: String,
        lhs: Vec<usize>,
        rhs: Vec<usize>,
    },
    Generic(String),
}

impl fmt::Display for FusionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FusionError::ShapeMismatch { op, lhs, rhs } => {
                write!(f, "Shape mismatch in {}: {:?} vs {:?}", op, lhs, rhs)
            }
            FusionError::Generic(msg) => write!(f, "Fusion Error: {}", msg),
        }
    }
}

impl std::error::Error for FusionError {}

pub type FusionResult<T> = Result<T, FusionError>;
