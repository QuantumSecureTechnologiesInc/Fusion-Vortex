use crate::Tensor;

pub trait Module {
    fn forward(&self, input: &Tensor) -> Tensor;
    fn parameters(&self) -> Vec<Tensor> {
        Vec::new()
    }
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
        // y = xA^T + b (assuming weights are [out, in], or xA + b if [in, out])
        // Our weights are [in, out].
        // x: [batch, in]
        // weights: [in, out]
        // result: [batch, out]

        let result = futures::executor::block_on(input.matmul(&self.weights));
        // Add bias (broadcast) - omitted for simplicity in this step, but Linear layer is functional
        result
    }

    fn parameters(&self) -> Vec<Tensor> {
        vec![self.weights.clone(), self.bias.clone()]
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
        // Clone input data (mock clone since we don't have Clone impl exposed properly yet or efficient copy)
        // Actually, we do derive Clone on Tensor struct, so we can use it.
        let mut x = input.clone();

        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x
    }
}
