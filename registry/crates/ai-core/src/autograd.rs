/// Stub implementations for Autograd Variable and SGD optimizer

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

pub struct SGD {
    pub lr: f64,
}

impl SGD {
    pub fn new(lr: f64) -> Self {
        Self { lr }
    }
}
