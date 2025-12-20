// Force recheck
use super::classical::ClassicalType;
use super::quantum::Qubit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorType {
    /// Shape of the tensor (e.g., [batch, channels, height, width])
    pub shape: Vec<usize>,
    /// Data type of tensor elements
    pub dtype: TensorDType,
    /// Device where tensor is stored
    pub device: String,
    /// Pointer to data in memory manager
    pub data_ptr: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TensorDType {
    F32,
    F64,
    I32,
    I64,
    U32,
    U64,
    Bool,
}

pub enum FusionType {
    Classical(ClassicalType),
    Tensor(TensorType),
    Quantum(Qubit),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HybridValue {
    Classical(ClassicalType),
    Tensor(TensorType),
    Quantum(Qubit),
    Vector(Vec<HybridValue>),
}
