//! N-dimensional tensor with compile-time rank enforcement
//! Integrated from fusion_core Tensor Types.rs

use crate::error::{TensorError, TensorResult};
use fusion_traits::{DataType, Numeric};
use std::marker::PhantomData;

/// N-dimensional array with compile-time rank enforcement
#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T: Numeric, const RANK: usize> {
    pub(crate) data: Vec<T>,
    pub(crate) shape: [usize; RANK],
    pub(crate) strides: [usize; RANK],
    pub(crate) dtype: DataType,
    pub(crate) _phantom: PhantomData<T>,
}

// Type aliases for common ranks
pub type Scalar<T> = Tensor<T, 0>;
pub type Vector<T> = Tensor<T, 1>;
pub type Matrix<T> = Tensor<T, 2>;

impl<T: Numeric, const RANK: usize> Tensor<T, RANK> {
    /// Create tensor filled with zeros
    pub fn zeros(shape: [usize; RANK]) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![T::zero(); size];
        let strides = Self::compute_strides(&shape);

        Tensor {
            data,
            shape,
            strides,
            dtype: T::data_type(),
            _phantom: PhantomData,
        }
    }

    /// Create tensor filled with ones
    pub fn ones(shape: [usize; RANK]) -> Self {
        let size: usize = shape.iter().product();
        let data = vec![T::one(); size];
        let strides = Self::compute_strides(&shape);

        Tensor {
            data,
            shape,
            strides,
            dtype: T::data_type(),
            _phantom: PhantomData,
        }
    }

    /// Create tensor from vector with shape validation
    pub fn from_vec(data: Vec<T>, shape: [usize; RANK]) -> TensorResult<Self> {
        let size: usize = shape.iter().product();
        if data.len() != size {
            return Err(TensorError::ShapeMismatch {
                op: "Tensor::from_vec".into(),
                lhs: vec![data.len()],
                rhs: vec![size],
            });
        }

        let strides = Self::compute_strides(&shape);
        Ok(Tensor {
            data,
            shape,
            strides,
            dtype: T::data_type(),
            _phantom: PhantomData,
        })
    }

    /// Compute row-major strides from shape
    fn compute_strides(shape: &[usize; RANK]) -> [usize; RANK] {
        let mut strides = [1; RANK];
        if RANK > 0 {
            for i in (0..RANK - 1).rev() {
                strides[i] = strides[i + 1] * shape[i + 1];
            }
        }
        strides
    }

    /// Get element with bounds checking
    pub fn get(&self, indices: [usize; RANK]) -> TensorResult<T> {
        let index = self.compute_flat_index(&indices)?;
        Ok(self.data[index])
    }

    /// Set element with bounds checking
    pub fn set(&mut self, indices: [usize; RANK], value: T) -> TensorResult<()> {
        let index = self.compute_flat_index(&indices)?;
        self.data[index] = value;
        Ok(())
    }

    /// Compute flat index from multi-dimensional indices
    fn compute_flat_index(&self, indices: &[usize; RANK]) -> TensorResult<usize> {
        let mut index = 0;
        for i in 0..RANK {
            if indices[i] >= self.shape[i] {
                return Err(TensorError::IndexOutOfBounds {
                    indices: indices.to_vec(),
                    shape: self.shape.to_vec(),
                });
            }
            index += indices[i] * self.strides[i];
        }
        Ok(index)
    }

    /// Get tensor shape
    pub fn shape(&self) -> &[usize; RANK] {
        &self.shape
    }

    /// Get total number of elements
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Get data type
    pub fn dtype(&self) -> DataType {
        self.dtype
    }
}

// Special implementations for Matrix (2D tensors)
impl<T: Numeric> Matrix<T> {
    /// Get matrix dimensions (rows, cols)
    pub fn dims(&self) -> (usize, usize) {
        (self.shape[0], self.shape[1])
    }

    /// Get element at (row, col)
    pub fn at(&self, row: usize, col: usize) -> TensorResult<T> {
        self.get([row, col])
    }

    ///Set element at (row, col)
    pub fn set_at(&mut self, row: usize, col: usize, value: T) -> TensorResult<()> {
        self.set([row, col], value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let t: Tensor<f64, 2> = Tensor::zeros([3, 4]);
        assert_eq!(t.shape(), &[3, 4]);
        assert_eq!(t.size(), 12);
    }

    #[test]
    fn test_tensor_from_vec() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let t = Tensor::from_vec(data, [2, 2]).unwrap();
        assert_eq!(t.get([0, 0]).unwrap(), 1.0);
        assert_eq!(t.get([1, 1]).unwrap(), 4.0);
    }

    #[test]
    fn test_tensor_get_set() {
        let mut t: Tensor<i32, 2> = Tensor::zeros([2, 2]);
        t.set([0, 1], 42).unwrap();
        assert_eq!(t.get([0, 1]).unwrap(), 42);
    }

    #[test]
    fn test_bounds_checking() {
        let t: Tensor<f64, 2> = Tensor::zeros([2, 2]);
        assert!(t.get([3, 0]).is_err());
    }

    #[test]
    fn test_matrix_ops() {
        let m: Matrix<f64> = Matrix::ones([3, 3]);
        assert_eq!(m.dims(), (3, 3));
        assert_eq!(m.at(1, 1).unwrap(), 1.0);
    }
}
