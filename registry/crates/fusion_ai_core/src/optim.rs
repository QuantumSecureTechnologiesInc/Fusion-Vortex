use crate::Tensor;

pub trait Optimizer {
    fn step(&mut self);
}

#[allow(dead_code)]
pub struct Adam {
    lr: f64,
    params: Vec<Tensor>,
}

impl Adam {
    pub fn new(params: Vec<Tensor>, lr: f64) -> Self {
        Self { lr, params }
    }
}

impl Optimizer for Adam {
    fn step(&mut self) {
        // Mock optimization step
    }
}

#[allow(dead_code)]
pub struct SGD {
    lr: f64,
    params: Vec<Tensor>,
}

impl SGD {
    pub fn new(params: Vec<Tensor>, lr: f64) -> Self {
        Self { lr, params }
    }
}

impl Optimizer for SGD {
    fn step(&mut self) {
        // Mock optimization step
    }
}
