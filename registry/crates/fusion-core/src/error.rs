use thiserror::Error;

#[derive(Error, Debug)]
pub enum FusionError {
    #[error("Fusion error: {0}")]
    Generic(String),
    #[error("Shape error: {0}")]
    ShapeError(String),
    #[error("Shape mismatch in {op}: lhs {lhs:?}, rhs {rhs:?}")]
    ShapeMismatch {
        op: String,
        lhs: Vec<usize>,
        rhs: Vec<usize>,
    },
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),
    #[error("Compilation error: {0}")]
    CompilationError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unknown variable: {0}")]
    UnknownVariable(String),
}

pub type FusionResult<T> = Result<T, FusionError>;
