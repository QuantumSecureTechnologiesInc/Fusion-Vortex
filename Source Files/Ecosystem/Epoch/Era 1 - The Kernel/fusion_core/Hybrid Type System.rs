/// Hybrid Type System definitions.
/// This module unifies Classical, Tensor, and Quantum types into a single hierarchy
/// for the compiler and runtime.

use crate::types::classical::ClassicalType;
use crate::types::tensor::{Tensor, DataType};
use crate::types::quantum::{Qubit, QubitRegister, QuantumCircuit, QuantumState};
use num_complex::Complex64;

/// Enum representing the Compile-Time type of any Fusion variable.
#[derive(Debug, Clone, PartialEq)]
pub enum FusionType {
    Classical(ClassicalType),
    Tensor(TensorTypeMeta),
    Quantum(QuantumTypeMeta),
    // A superposition of types (future feature for quantum control flow)
    Hybrid(Box<FusionType>, Box<FusionType>), 
}

#[derive(Debug, Clone, PartialEq)]
pub struct TensorTypeMeta {
    pub dtype: DataType,
    pub rank: usize,
    // Shape might be unknown at compile time in some dynamic languages, 
    // but Fusion enforces static shape checking where possible.
    pub shape: Option<Vec<usize>>, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuantumTypeMeta {
    Qubit,
    Register(usize), // Size known
    Circuit,
    State,
}

/// Enum representing the Runtime value of any Fusion variable.
/// This is the "Any" type for the interpreter.
#[derive(Debug)]
pub enum HybridValue {
    // Classical Wrappers
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    
    // Tensor Wrappers (Simplified for the core implementation)
    // In a real implementation, we would use a Box<dyn TensorTrait> or similar dynamic dispatch.
    // Here we wrap specific instances for demonstration.
    ScalarF64(Tensor<f64, 0>),
    MatrixF64(Tensor<f64, 2>),
    
    // Quantum Wrappers
    Qubit(Qubit),
    Register(QubitRegister),
    Circuit(QuantumCircuit),
    State(QuantumState),
}

impl FusionType {
    /// Check if this type can be implicitly converted to another.
    /// Fusion allows VERY few implicit conversions to ensure safety.
    pub fn can_cast_to(&self, other: &FusionType) -> bool {
        match (self, other) {
            // Exact match
            (a, b) if a == b => true,
            
            // Classical -> Tensor (Scalar promotion)
            (FusionType::Classical(ClassicalType::Float(_)), FusionType::Tensor(t)) => {
                t.rank == 0 && matches!(t.dtype, DataType::Float64)
            },
            
            // No implicit Quantum conversions allowed
            (FusionType::Quantum(_), _) => false,
            (_, FusionType::Quantum(_)) => false,
            
            _ => false
        }
    }
}