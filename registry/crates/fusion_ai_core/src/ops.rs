use crate::Tensor;

pub fn matmul(a: &Tensor, b: &Tensor) -> Tensor {
    // Placeholder for operation dispatch
    let rows = a.shape()[0];
    let cols = b.shape()[1];
    Tensor::zeros(vec![rows, cols])
}
