use crate::types::tensor::Tensor;
use num_complex::Complex64;

pub trait Numeric {}
impl Numeric for i64 {}
impl Numeric for f64 {}
impl Numeric for Complex64 {}

pub trait Unitary {
    fn matrix(&self) -> Tensor<Complex64, 2>;
}

pub trait Measurable {
    fn measure(&self) -> u8;
}
