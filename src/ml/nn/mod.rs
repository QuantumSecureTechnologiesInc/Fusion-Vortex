// src/ml/nn/mod.rs - Neural Network Module
#![allow(dead_code)]
// Provides layers, loss functions, and models

pub mod layers;

use super::tensor::Tensor;
use super::MLError;

/// Neural Network Module Trait
pub trait Module {
    /// Forward pass
    fn forward(&self, input: &Tensor) -> Result<Tensor, MLError>;

    /// Get trainable parameters
    fn parameters(&self) -> Vec<&Tensor>;

    /// Set training mode
    fn train(&mut self, mode: bool);
}
