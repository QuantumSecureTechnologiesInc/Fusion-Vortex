// __FU_COMPAT_START__
#![allow(missing_docs)]
#[allow(missing_docs, dead_code)] type FString = String;
#[allow(missing_docs, dead_code)] type FVec<T> = Vec<T>;
// __FU_COMPAT_END__
/// LLM Inference Engine
use fusion_core::types::tensor::Matrix;
use fusion_core::{FusionError, FusionResult};
/// Represents a loaded model ready for inference
pub struct InferenceEngine {
    pub model_path: FString,
    pub layers: FVec<LinearLayer>,
}
/// Basic Linear Layer (Weights + Bias)
pub struct LinearLayer {
    pub weights: Matrix<f64>,
    pub bias: Option<Matrix<f64>>,
}
impl InferenceEngine {
    pub fn new(model_path: FString) -> Self {
        Self {
            model_path,
            layers: Vec::new(),
        }
    }
    /// Add a layer to the model (for construction)
    pub fn add_layer(&mut self, weights: Matrix<f64>, bias: Option<Matrix<f64>>) {
        self.layers.push(LinearLayer { weights, bias });
    }
    /// Run inference (forward pass) on input tensor
    pub fn infer(&self, input: &Matrix<f64>) -> FusionResult<Matrix<f64>> {
        let mut current = input.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            current = self.matmul(&current, &layer.weights)?;
            if let Some(bias) = &layer.bias {
                current = self.add(&current, bias)?;
            }
            if i < self.layers.len() - 1 {
                current = self.relu(&current);
            }
        }
        Ok(current)
    }
    fn matmul(&self, a: &Matrix<f64>, b: &Matrix<f64>) -> FusionResult<Matrix<f64>> {
        if a.shape().len() != 2 || b.shape().len() != 2 || a.shape()[1] != b.shape()[0] {
            return Err(FusionError::ShapeMismatch {
                op: "matmul".to_string(),
                lhs: a.shape().to_vec(),
                rhs: b.shape().to_vec(),
            });
        }
        a.matmul(b)
    }
    fn add(&self, a: &Matrix<f64>, b: &Matrix<f64>) -> FusionResult<Matrix<f64>> {
        if a.shape() != b.shape() {
            return Err(FusionError::ShapeMismatch {
                op: "add".to_string(),
                lhs: a.shape().to_vec(),
                rhs: b.shape().to_vec(),
            });
        }
        let data: FVec<f64> = a
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(x, y)| x + y)
            .collect();
        let shape = [a.shape()[0], a.shape()[1]];
        Ok(Matrix::new(data, shape).unwrap())
    }
    fn relu(&self, a: &Matrix<f64>) -> Matrix<f64> {
        let data: FVec<f64> = a
            .data
            .iter()
            .map(|&x| if x > 0.0 { x } else { 0.0 })
            .collect();
        let shape = [a.shape()[0], a.shape()[1]];
        Matrix::new(data, shape).unwrap()
    }
}
