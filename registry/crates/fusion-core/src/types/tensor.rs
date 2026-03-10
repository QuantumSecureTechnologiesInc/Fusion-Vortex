use crate::{FusionError, FusionResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T, const RANK: usize> {
    pub data: Vec<T>,
    shape: [usize; RANK],
}

pub type Matrix<T> = Tensor<T, 2>;

impl<T: Clone + Default, const RANK: usize> Tensor<T, RANK> {
    pub fn zeros(shape: [usize; RANK]) -> Self {
        let len = shape.iter().copied().product::<usize>();
        Self {
            data: vec![T::default(); len],
            shape,
        }
    }
}

impl<T, const RANK: usize> Tensor<T, RANK> {
    pub fn new(data: Vec<T>, shape: [usize; RANK]) -> FusionResult<Self> {
        let expected = shape.iter().copied().product::<usize>();
        if data.len() != expected {
            return Err(FusionError::ShapeError(format!(
                "invalid tensor length: got {}, expected {}",
                data.len(),
                expected
            )));
        }
        Ok(Self { data, shape })
    }

    pub fn shape(&self) -> &[usize; RANK] {
        &self.shape
    }
}

impl Matrix<f64> {
    pub fn matmul(&self, rhs: &Self) -> FusionResult<Self> {
        let [m, k_lhs] = self.shape;
        let [k_rhs, n] = rhs.shape;
        if k_lhs != k_rhs {
            return Err(FusionError::ShapeMismatch {
                op: "matmul".to_string(),
                lhs: self.shape.to_vec(),
                rhs: rhs.shape.to_vec(),
            });
        }

        let mut out = vec![0.0; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut acc = 0.0;
                for k in 0..k_lhs {
                    acc += self.data[i * k_lhs + k] * rhs.data[k * n + j];
                }
                out[i * n + j] = acc;
            }
        }

        Ok(Self {
            data: out,
            shape: [m, n],
        })
    }

    pub fn add(&self, rhs: &Self) -> FusionResult<Self> {
        if self.shape != rhs.shape {
            return Err(FusionError::ShapeMismatch {
                op: "add".to_string(),
                lhs: self.shape.to_vec(),
                rhs: rhs.shape.to_vec(),
            });
        }

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(l, r)| l + r)
            .collect::<Vec<_>>();

        Ok(Self {
            data,
            shape: self.shape,
        })
    }

    pub fn sub(&self, rhs: &Self) -> FusionResult<Self> {
        if self.shape != rhs.shape {
            return Err(FusionError::ShapeMismatch {
                op: "sub".to_string(),
                lhs: self.shape.to_vec(),
                rhs: rhs.shape.to_vec(),
            });
        }

        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(l, r)| l - r)
            .collect::<Vec<_>>();

        Ok(Self {
            data,
            shape: self.shape,
        })
    }

    pub fn scale(&self, factor: f64) -> Self {
        let data = self.data.iter().map(|v| v * factor).collect::<Vec<_>>();
        Self {
            data,
            shape: self.shape,
        }
    }
}
