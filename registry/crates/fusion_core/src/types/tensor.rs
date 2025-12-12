use crate::FusionResult;
use serde::{Deserialize, Serialize};

/// A multi-dimensional array (Tensor) implementing the HAFT interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
}

pub type Matrix<T> = Tensor<T>;

impl<T> Tensor<T> {
    pub fn new(data: Vec<T>, shape: impl Into<Vec<usize>>) -> FusionResult<Self> {
        Ok(Self {
            data,
            shape: shape.into(),
        })
    }
}

// Matrix specific implementations
impl<T> Tensor<T>
where
    T: Clone,
{
    pub fn set(&mut self, _index: [usize; 2], _value: T) -> FusionResult<()> {
        // Stub implementation for now - just needed for compilation
        Ok(())
    }
}
