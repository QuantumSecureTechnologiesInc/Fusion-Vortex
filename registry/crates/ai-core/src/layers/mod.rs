pub mod linear;

use crate::autograd::Variable;
use fusion_core::FusionResult;
pub use linear::Linear;

/// Base trait for all neural network layers
pub trait Layer {
    /// Forward pass of the layer
    fn forward(&self, input: &Variable) -> FusionResult<Variable>;

    /// Returns the trainable parameters of the layer
    fn parameters(&self) -> Vec<&Variable>;

    /// Returns the trainable parameters of the layer (mutable)
    fn parameters_mut(&mut self) -> Vec<&mut Variable>;
}
