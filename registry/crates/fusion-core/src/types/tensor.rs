use crate::FusionError;
use crate::FusionResult;
use ndarray::{Array, ArrayD, IxDyn};
use num_complex::Complex64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Tensor<T, const RANK: usize> {
    pub data: ArrayD<T>,
}

pub type Matrix<T> = Tensor<T, 2>;
pub type Vector1D<T> = Tensor<T, 1>;
pub type Tensor3D<T> = Tensor<T, 3>;

impl<T, const RANK: usize> Tensor<T, RANK> {
    pub fn from_array(data: ArrayD<T>) -> Self {
        Self { data }
    }

    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    pub fn get<I>(&self, index: I) -> Option<&T>
    where
        I: ndarray::NdIndex<ndarray::Dim<ndarray::IxDynImpl>>,
    {
        self.data.get(index)
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
    where
        I: ndarray::NdIndex<ndarray::Dim<ndarray::IxDynImpl>>,
    {
        self.data.get_mut(index)
    }

    pub fn set<I>(&mut self, index: I, value: T) -> FusionResult<()>
    where
        I: ndarray::NdIndex<ndarray::Dim<ndarray::IxDynImpl>> + Clone + std::fmt::Debug,
    {
        if let Some(elem) = self.data.get_mut(index.clone()) {
            *elem = value;
            Ok(())
        } else {
            Err(FusionError::ShapeError(format!(
                "Index {:?} out of bounds",
                index
            )))
        }
    }

    pub fn mapv<F>(&self, f: F) -> Self
    where
        T: Clone,
        F: FnMut(T) -> T,
    {
        Self {
            data: self.data.mapv(f),
        }
    }
}

// Constructor for Matrix (Rank 2)
impl<T: Clone> Tensor<T, 2> {
    pub fn new(data: Vec<T>, shape: [usize; 2]) -> FusionResult<Self> {
        let arr = Array::from_shape_vec(IxDyn(&shape), data)
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        Ok(Self { data: arr })
    }

    pub fn from_vec(data: Vec<T>, shape: [usize; 2]) -> FusionResult<Self> {
        Self::new(data, shape)
    }
}

// Constructor for Vector (Rank 1)
impl<T: Clone> Tensor<T, 1> {
    pub fn new(data: Vec<T>, shape: [usize; 1]) -> FusionResult<Self> {
        let arr = Array::from_shape_vec(IxDyn(&shape), data)
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        Ok(Self { data: arr })
    }

    pub fn from_vec(data: Vec<T>, shape: [usize; 1]) -> FusionResult<Self> {
        Self::new(data, shape)
    }
}

// Constructor for Tensor 3D
impl<T: Clone> Tensor<T, 3> {
    pub fn new(data: Vec<T>, shape: [usize; 3]) -> FusionResult<Self> {
        let arr = Array::from_shape_vec(IxDyn(&shape), data)
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        Ok(Self { data: arr })
    }

    pub fn from_vec(data: Vec<T>, shape: [usize; 3]) -> FusionResult<Self> {
        Self::new(data, shape)
    }
}

// Generic zeros and ones
impl<T: Clone + num_traits::Zero, const RANK: usize> Tensor<T, RANK> {
    pub fn zeros(shape: [usize; RANK]) -> Self {
        let arr = Array::zeros(IxDyn(&shape));
        Self { data: arr }
    }
}

impl<T: Clone + num_traits::One, const RANK: usize> Tensor<T, RANK> {
    pub fn ones(shape: [usize; RANK]) -> Self {
        let arr = Array::ones(IxDyn(&shape));
        Self { data: arr }
    }
}

// Dot / Matmul / Transpose for Matrix f64
impl Tensor<f64, 2> {
    pub fn dot(&self, other: &Self) -> FusionResult<Self> {
        let s_shape = self.shape();
        let o_shape = other.shape();
        if s_shape[1] != o_shape[0] {
            return Err(FusionError::ShapeError(format!(
                "Incompatible shapes for dot product: {:?} vs {:?}",
                s_shape, o_shape
            )));
        }

        let a = self
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        let b = other
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        let res = a.dot(&b);
        Ok(Self::from_array(res.into_dyn()))
    }

    pub fn matmul(&self, other: &Self) -> FusionResult<Self> {
        self.dot(other)
    }

    pub fn transpose(&self) -> FusionResult<Self> {
        let transposed = self
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|e| FusionError::ShapeError(e.to_string()))?
            .t()
            .to_owned();
        Ok(Self::from_array(transposed.into_dyn()))
    }

    pub fn add(&self, other: &Self) -> FusionResult<Self> {
        if self.shape() != other.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "add".to_string(),
                lhs: self.shape().to_vec(),
                rhs: other.shape().to_vec(),
            });
        }
        Ok(Self::from_array(&self.data + &other.data))
    }

    pub fn sub(&self, other: &Self) -> FusionResult<Self> {
        if self.shape() != other.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "sub".to_string(),
                lhs: self.shape().to_vec(),
                rhs: other.shape().to_vec(),
            });
        }
        Ok(Self::from_array(&self.data - &other.data))
    }

    pub fn mul(&self, other: &Self) -> FusionResult<Self> {
        if self.shape() != other.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "mul".to_string(),
                lhs: self.shape().to_vec(),
                rhs: other.shape().to_vec(),
            });
        }
        Ok(Self::from_array(&self.data * &other.data))
    }

    pub fn div(&self, other: &Self) -> FusionResult<Self> {
        if self.shape() != other.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "div".to_string(),
                lhs: self.shape().to_vec(),
                rhs: other.shape().to_vec(),
            });
        }
        Ok(Self::from_array(&self.data / &other.data))
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self::from_array(&self.data.clone() * scalar)
    }
}

// Dot / Matmul / Transpose for Matrix Complex64
impl Tensor<Complex64, 2> {
    pub fn dot(&self, other: &Self) -> FusionResult<Self> {
        let s_shape = self.shape();
        let o_shape = other.shape();
        if s_shape[1] != o_shape[0] {
            return Err(FusionError::ShapeError(format!(
                "Incompatible shapes for dot product: {:?} vs {:?}",
                s_shape, o_shape
            )));
        }

        let a = self
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        let b = other
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|e| FusionError::ShapeError(e.to_string()))?;
        let res = a.dot(&b);
        Ok(Self::from_array(res.into_dyn()))
    }

    pub fn matmul(&self, other: &Self) -> FusionResult<Self> {
        self.dot(other)
    }

    pub fn transpose(&self) -> FusionResult<Self> {
        let transposed = self
            .data
            .view()
            .into_dimensionality::<ndarray::Ix2>()
            .map_err(|e| FusionError::ShapeError(e.to_string()))?
            .t()
            .to_owned();
        Ok(Self::from_array(transposed.into_dyn()))
    }

    pub fn add(&self, other: &Self) -> FusionResult<Self> {
        if self.shape() != other.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "add".to_string(),
                lhs: self.shape().to_vec(),
                rhs: other.shape().to_vec(),
            });
        }
        Ok(Self::from_array(&self.data + &other.data))
    }

    pub fn sub(&self, other: &Self) -> FusionResult<Self> {
        if self.shape() != other.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "sub".to_string(),
                lhs: self.shape().to_vec(),
                rhs: other.shape().to_vec(),
            });
        }
        Ok(Self::from_array(&self.data - &other.data))
    }

    pub fn scale(&self, scalar: Complex64) -> Self {
        Self::from_array(&self.data.clone() * scalar)
    }
}

// Implement std::ops traits for ergonomic usage
impl Add for &Tensor<f64, 2> {
    type Output = FusionResult<Tensor<f64, 2>>;
    fn add(self, other: Self) -> Self::Output {
        self.add(other)
    }
}

impl Sub for &Tensor<f64, 2> {
    type Output = FusionResult<Tensor<f64, 2>>;
    fn sub(self, other: Self) -> Self::Output {
        self.sub(other)
    }
}

impl Mul for &Tensor<f64, 2> {
    type Output = FusionResult<Tensor<f64, 2>>;
    fn mul(self, other: Self) -> Self::Output {
        self.mul(other)
    }
}

impl Div for &Tensor<f64, 2> {
    type Output = FusionResult<Tensor<f64, 2>>;
    fn div(self, other: Self) -> Self::Output {
        self.div(other)
    }
}

impl Add for &Tensor<Complex64, 2> {
    type Output = FusionResult<Tensor<Complex64, 2>>;
    fn add(self, other: Self) -> Self::Output {
        self.add(other)
    }
}

impl Sub for &Tensor<Complex64, 2> {
    type Output = FusionResult<Tensor<Complex64, 2>>;
    fn sub(self, other: Self) -> Self::Output {
        self.sub(other)
    }
}
