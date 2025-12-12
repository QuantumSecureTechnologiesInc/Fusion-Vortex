use crate::Tensor;

pub struct Adam {
    lr: f64,
}

impl Adam {
    pub fn new(lr: f64) -> Self {
        Self { lr }
    }

    pub fn step(&self, _params: &mut [Tensor]) {
        // Mock optimization step
    }
}
