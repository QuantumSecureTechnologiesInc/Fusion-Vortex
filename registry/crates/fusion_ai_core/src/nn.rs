use crate::Tensor;

pub trait Module {
    fn forward(&self, input: &Tensor) -> Tensor;
}

pub struct Linear {
    pub weights: Tensor,
    pub bias: Tensor,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        // In a real impl, these would be random initialized
        Self {
            weights: Tensor::ones(vec![in_features, out_features]),
            bias: Tensor::zeros(vec![out_features]),
        }
    }
}

impl Module for Linear {
    fn forward(&self, input: &Tensor) -> Tensor {
        // y = xA + b
        // For simulation, we just return input * 0.1 (mock linear)
        // because we don't have a full tensor math engine yet.
        // But we return the correct shape.
        let batch_size = input.shape()[0];
        let out_features = self.weights.shape()[1];

        Tensor::zeros(vec![batch_size, out_features])
    }
}

pub struct ReLU;

impl ReLU {
    pub fn new() -> Self {
        Self
    }
}

impl Module for ReLU {
    fn forward(&self, input: &Tensor) -> Tensor {
        // Mock ReLU: return max(0, x)
        // Since we don't have tensor iterators yet, returning explicit new tensor
        Tensor::zeros(input.shape().to_vec())
    }
}

pub struct Sequential {
    layers: Vec<Box<dyn Module>>,
}

impl Sequential {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add<M: Module + 'static>(mut self, layer: M) -> Self {
        self.layers.push(Box::new(layer));
        self
    }
}

impl Module for Sequential {
    fn forward(&self, input: &Tensor) -> Tensor {
        let mut x = Tensor::zeros(input.shape().to_vec()); // Should be clone, mock for now

        // This is a mock forward pass because Tensor doesn't support Clone yet
        for layer in &self.layers {
            x = layer.forward(&x); // In real impl, this chains correctly
        }
        x
    }
}
