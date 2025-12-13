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
    T: Clone + Default,
{
    pub fn zeros(shape: impl Into<Vec<usize>>) -> FusionResult<Self> {
        let shape = shape.into();
        let size = shape.iter().product();
        let data = vec![T::default(); size];
        Self::new(data, shape)
    }

    pub fn set(&mut self, _index: [usize; 2], _value: T) -> FusionResult<()> {
        // Stub implementation for now - just needed for compilation
        Ok(())
    }

    pub fn get(&self, _indices: &[usize]) -> Option<&T> {
        // Stub implementation
        self.data.first()
    }

    pub fn matmul(&self, _other: &Self) -> FusionResult<Self> {
        // Stub implementation
        Ok(self.clone())
    }

    pub fn transpose(&self) -> FusionResult<Self> {
        // Stub implementation
        Ok(self.clone())
    }
}

impl<T> std::ops::Add for Tensor<T>
where
    T: Clone + Copy + std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, _other: Self) -> Self {
        // Stub implementation
        self
    }
}

pub type Vector1D<T> = Tensor<T>;
pub type Tensor3D<T> = Tensor<T>;
