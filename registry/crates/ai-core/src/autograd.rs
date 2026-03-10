// __FU_COMPAT_START__
#![allow(missing_docs)]
#[allow(missing_docs, dead_code)] type FString = String;
#[allow(missing_docs, dead_code)] type FSize = usize;
// __FU_COMPAT_END__
use fusion_core::types::tensor::Tensor;
use fusion_core::FusionResult;
/// Production Autograd Variable
///
/// Wraps a tensor with optional name and metadata.
/// In a full autograd system, this would also track the computational graph
/// and gradients. For now, we providing a robust foundation that stores the actual tensor data.
#[derive(Clone, Debug)]
pub struct Variable {
    pub name: FString,
    pub data: Tensor<f64, 2>,
    pub grad: Option<Tensor<f64, 2>>,
}
impl Variable {
    /// Create a new variable with data and a name
    pub fn new(name: impl Into<FString>, data: Tensor<f64, 2>) -> Self {
        Self {
            name: name.into(),
            data,
            grad: None,
        }
    }
    /// Helper to create a variable from an existing name (compatibility path)
    pub fn from_name(name: impl Into<FString>) -> Self {
        Self {
            name: name.into(),
            data: Tensor::zeros([1, 1]),
            grad: None,
        }
    }
    /// Access the shape of the underlying tensor
    pub fn shape(&self) -> &[FSize] {
        self.data.shape()
    }
}
/// Stochastic Gradient Descent (SGD) Optimizer
///
/// Updates parameters based on their gradients and a learning rate.
pub struct SGD {
    pub lr: f64,
}
impl SGD {
    pub fn new(lr: f64) -> Self {
        Self { lr }
    }
    /// Update a single variable using its gradient
    pub fn step(&self, var: &mut Variable) -> FusionResult<()> {
        if let Some(grad) = &var.grad {
            let update = grad.scale(self.lr);
            var.data = var.data.sub(&update)?;
            var.grad = None;
        }
        Ok(())
    }
    /// Update multiple variables
    pub fn step_many(&self, vars: &mut [Variable]) -> FusionResult<()> {
        for var in vars {
            self.step(var)?;
        }
        Ok(())
    }
}
