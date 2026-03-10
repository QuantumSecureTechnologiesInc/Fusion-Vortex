#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FSize = FSize;
#[allow(missing_docs, dead_code)]
type FVec<T> = FVec<T>;
use super::classical::ClassicalType;
use super::quantum::Qubit;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TensorType {
    /// Shape of the tensor (e.g., [batch, channels, height, width])
    pub shape: FVec<FSize>,
    /// Data type of tensor elements
    pub dtype: TensorDType,
    /// Device where tensor is stored
    pub device: FString,
    /// Pointer to data in memory manager
    pub data_ptr: FSize,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum TensorDType {
    F32,
    F64,
    I32,
    I64,
    U32,
    U64,
    Bool,
}
enum FusionType {
    Classical(ClassicalType),
    Tensor(TensorType),
    Quantum(Qubit),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum HybridValue {
    Classical(ClassicalType),
    Tensor(TensorType),
    Quantum(Qubit),
    Vector(FVec<HybridValue>),
}
