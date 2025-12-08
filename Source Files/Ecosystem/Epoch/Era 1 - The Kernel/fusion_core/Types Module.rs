pub mod classical;
pub mod tensor;
pub mod quantum; 
pub mod hybrid; // Phase 4: Now active

pub use classical::{ClassicalType, IntType, FloatType, StructType, EnumType};
pub use tensor::{Tensor, Matrix, Vector1D, Scalar, DataType};
pub use quantum::{Qubit, QubitRegister, QuantumGate, QuantumCircuit, QuantumState};
pub use hybrid::{FusionType, HybridValue, TensorTypeMeta, QuantumTypeMeta};