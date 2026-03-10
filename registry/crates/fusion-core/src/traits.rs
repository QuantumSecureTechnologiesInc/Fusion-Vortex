#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FI64 = FI64;
use crate::types::tensor::Tensor;
use num_complex::Complex64;
trait Numeric {}
impl Numeric for FI64 {}
impl Numeric for f64 {}
impl Numeric for Complex64 {}
trait Unitary {
    fn matrix(&self) -> Tensor<Complex64, 2>;
}
trait Measurable {
    fn measure(&self) -> u8;
}
