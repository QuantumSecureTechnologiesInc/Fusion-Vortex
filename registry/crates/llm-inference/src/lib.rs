/// LLM Inference Engine
use fusion_core_compiler::types::tensor::Matrix;
use fusion_core_compiler::{FusionError, FusionResult};

/// Represents a loaded model ready for inference
pub struct InferenceEngine {
    pub model_path: String,
    pub layers: Vec<LinearLayer>,
}

/// Basic Linear Layer (Weights + Bias)
pub struct LinearLayer {
    pub weights: Matrix<f64>,
    pub bias: Option<Matrix<f64>>,
}

impl InferenceEngine {
    pub fn new(model_path: String) -> Self {
        // In a real scenario, this would load from disk.
        // initialized with empty layers for now, but the structure is real.
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
            // Matrix multiplication: current * weights
            // Note: Simplistic matmul, ignoring broadcasting for this implementation
            current = self.matmul(&current, &layer.weights)?;

            // Add bias if present
            if let Some(bias) = &layer.bias {
                current = self.add(&current, bias)?;
            }

            // Apply activation (ReLU) except for last layer
            if i < self.layers.len() - 1 {
                current = self.relu(&current);
            }
        }

        Ok(current)
    }

    fn matmul(&self, a: &Matrix<f64>, b: &Matrix<f64>) -> FusionResult<Matrix<f64>> {
        // Validation
        if a.shape.len() != 2 || b.shape.len() != 2 || a.shape[1] != b.shape[0] {
            return Err(FusionError::ShapeMismatch {
                op: "matmul".into(),
                lhs: a.shape.clone(),
                rhs: b.shape.clone(),
            });
        }

        let m = a.shape[0];
        let n = a.shape[1];
        let p = b.shape[1];
        let mut result_data = vec![0.0; m * p];

        // Naive O(n^3) matmul
        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += a.data[i * n + k] * b.data[k * p + j];
                }
                result_data[i * p + j] = sum;
            }
        }

        Ok(Matrix {
            data: result_data,
            shape: vec![m, p],
        })
    }

    fn add(&self, a: &Matrix<f64>, b: &Matrix<f64>) -> FusionResult<Matrix<f64>> {
        if a.shape != b.shape {
            return Err(FusionError::ShapeMismatch {
                op: "add".into(),
                lhs: a.shape.clone(),
                rhs: b.shape.clone(),
            });
        }

        let data: Vec<f64> = a
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(x, y)| x + y)
            .collect();
        Ok(Matrix {
            data,
            shape: a.shape.clone(),
        })
    }

    fn relu(&self, a: &Matrix<f64>) -> Matrix<f64> {
        let data: Vec<f64> = a
            .data
            .iter()
            .map(|&x| if x > 0.0 { x } else { 0.0 })
            .collect();
        Matrix {
            data,
            shape: a.shape.clone(),
        }
    }
}

