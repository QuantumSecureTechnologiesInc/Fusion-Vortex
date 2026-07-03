/// Production Model Graph Builder.
/// 
/// constructs models ensuring architectural validity (e.g. dimensions match).

use fusion_ai_core::{Layer, Linear, Variable};
use fusion_core::FusionResult;
use fusion_core::FusionError;

pub struct LayerConfig {
    pub input_dim: usize,
    pub output_dim: usize,
    pub layer_type: String, // "Linear", "ReLU", "Norm"
}

pub struct ModelGraph {
    layers: Vec<Box<dyn Layer>>,
    configs: Vec<LayerConfig>,
}

impl ModelGraph {
    pub fn new() -> Self {
        Self { layers: Vec::new(), configs: Vec::new() }
    }

    pub fn add_linear(mut self, in_dim: usize, out_dim: usize) -> FusionResult<Self> {
        // Validate connectivity
        if let Some(prev) = self.configs.last() {
            if prev.output_dim != in_dim {
                return Err(FusionError::ShapeMismatch {
                    op: "Builder::add_linear".into(),
                    lhs: vec![prev.output_dim],
                    rhs: vec![in_dim],
                });
            }
        }

        self.layers.push(Box::new(Linear::new(in_dim, out_dim)));
        self.configs.push(LayerConfig {
            input_dim: in_dim,
            output_dim: out_dim,
            layer_type: "Linear".into(),
        });

        Ok(self)
    }

    pub fn add_residual_block(mut self, internal_dim: usize) -> FusionResult<Self> {
        let current_dim = self.configs.last().map(|c| c.output_dim).unwrap_or(internal_dim);
        
        if current_dim != internal_dim {
             return Err(FusionError::InvalidDimension(
                 format!("Residual block expects input dim {}, got {}", internal_dim, current_dim)
             ));
        }

        // Logic to wrap layers in a Residual wrapper...
        // self.layers.push(Box::new(ResidualWrapper::new(...)));
        
        Ok(self)
    }

    pub fn build(self) -> Vec<Box<dyn Layer>> {
        self.layers
    }
}

