use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum FusionError {
    #[error("{0}")]
    Generic(String),
    #[error("{0}")]
    ShapeError(String),
    #[error("{0}")]
    InvalidDimension(String),
    #[error("{op} shape mismatch: lhs={lhs:?}, rhs={rhs:?}")]
    ShapeMismatch {
        op: String,
        lhs: Vec<usize>,
        rhs: Vec<usize>,
    },
}

pub type FusionResult<T> = Result<T, FusionError>;
