use crate::autograd::Variable;
use crate::layers::Layer;
use fusion_core::types::tensor::Tensor;
use fusion_core::FusionResult;

/// Production Linear (Dense) Layer
///
/// Implements y = x @ W + b
pub struct Linear {
    pub weight: Variable,
    pub bias: Option<Variable>,
    _in_features: usize,
    _out_features: usize,
}

impl Linear {
    /// Create a new linear layer
    ///
    /// # Arguments
    /// * `in_features` - Size of each input sample
    /// * `out_features` - Size of each output sample
    /// * `bias` - Whether the layer uses a bias vector
    pub fn new(in_features: usize, out_features: usize, has_bias: bool) -> FusionResult<Self> {
        // Initialize weights (normally small random, here zeros as placeholder for weight init strategy)
        let weight_data = Tensor::zeros([in_features, out_features]);
        let weight = Variable::new("weight", weight_data);

        let bias = if has_bias {
            let bias_data = Tensor::zeros([1, out_features]);
            Some(Variable::new("bias", bias_data))
        } else {
            None
        };

        Ok(Self {
            weight,
            bias,
            _in_features: in_features,
            _out_features: out_features,
        })
    }
}

impl Layer for Linear {
    fn forward(&self, input: &Variable) -> FusionResult<Variable> {
        // Perform matrix multiplication: y = input @ weight
        let mut output_data = input.data.matmul(&self.weight.data)?;

        // Add bias if present
        if let Some(bias) = &self.bias {
            output_data = output_data.add(&bias.data)?;
        }

        Ok(Variable::new("linear_output", output_data))
    }

    fn parameters(&self) -> Vec<&Variable> {
        let mut params = vec![&self.weight];
        if let Some(bias) = &self.bias {
            params.push(bias);
        }
        params
    }

    fn parameters_mut(&mut self) -> Vec<&mut Variable> {
        let mut params = vec![&mut self.weight];
        if let Some(bias) = &mut self.bias {
            params.push(bias);
        }
        params
    }
}
