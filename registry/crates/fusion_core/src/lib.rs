//! # Fusion Core
//!
//! Core type system and abstractions for the Fusion Programming Language.
//!
//! This crate provides the `FusionType` enum, which represents the fundamental
//! hybrid data type that can seamlessly transition between Classical, Tensor,
//! and Quantum representations.

use serde::{Deserialize, Serialize};
use std::fmt;

pub mod error;
pub mod types;

pub use error::{FusionError, FusionResult};
pub use types::tensor::{Matrix, Tensor};

/// The fundamental hybrid type in Fusion that can represent Classical, Tensor, or Quantum data.
///
/// # Design Philosophy
///
/// `FusionType` enables zero-cost abstractions across three computational domains:
/// - **Classical**: Traditional CPU-based scalar and aggregate types
/// - **Tensor**: GPU-accelerated multi-dimensional arrays for AI/ML
/// - **Quantum**: QPU-based quantum state representations
///
/// The runtime (`fusion_runtime_core`) handles transparent transitions between these
/// representations based on the execution context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionType {
    /// Classical scalar types
    Classical(ClassicalType),

    /// Tensor types for AI/ML workloads
    Tensor(TensorType),

    /// Quantum types for QPU execution
    Quantum(QuantumType),

    /// Hybrid type currently in transition between representations
    Transitioning { from: Box<FusionType>, to: TypeHint },
}

/// Classical data types (CPU-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassicalType {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<FusionType>),
    Struct(Vec<(String, FusionType)>),
}

/// Tensor metadata and reference to GPU memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorType {
    /// Shape of the tensor (e.g., [batch, channels, height, width])
    pub shape: Vec<usize>,

    /// Data type of tensor elements
    pub dtype: TensorDType,

    /// Device location (e.g., "cuda:0", "metal:0", "cpu")
    pub device: String,

    /// Pointer to the actual data (managed by fusion_runtime_mem_mgr)
    /// In a real implementation, this would be a raw pointer or handle
    pub data_ptr: usize,
}

/// Tensor data types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TensorDType {
    F32,
    F64,
    I32,
    I64,
    U8,
    Bool,
}

/// Quantum state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumType {
    /// Number of qubits
    pub num_qubits: usize,

    /// State vector (for simulation) or handle to QPU state
    pub state: QuantumState,

    /// QPU device identifier
    pub qpu_device: String,
}

/// Quantum state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumState {
    /// Classical simulation of quantum state (amplitude vector)
    Simulated(Vec<(f64, f64)>), // Complex numbers as (real, imag)

    /// Reference to actual QPU hardware state
    Hardware { job_id: String, provider: String },
}

/// Type hint for runtime optimization
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TypeHint {
    Classical,
    Tensor,
    Quantum,
}

impl FusionType {
    /// Create a new classical integer
    pub fn int(value: i64) -> Self {
        FusionType::Classical(ClassicalType::Int(value))
    }

    /// Create a new classical float
    pub fn float(value: f64) -> Self {
        FusionType::Classical(ClassicalType::Float(value))
    }

    /// Create a new classical boolean
    pub fn bool(value: bool) -> Self {
        FusionType::Classical(ClassicalType::Bool(value))
    }

    /// Create a new classical string
    pub fn string(value: impl Into<String>) -> Self {
        FusionType::Classical(ClassicalType::String(value.into()))
    }

    /// Get the current type hint for this value
    pub fn type_hint(&self) -> TypeHint {
        match self {
            FusionType::Classical(_) => TypeHint::Classical,
            FusionType::Tensor(_) => TypeHint::Tensor,
            FusionType::Quantum(_) => TypeHint::Quantum,
            FusionType::Transitioning { to, .. } => *to,
        }
    }

    /// Check if this type is currently transitioning
    pub fn is_transitioning(&self) -> bool {
        matches!(self, FusionType::Transitioning { .. })
    }
}

impl fmt::Display for FusionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FusionType::Classical(ct) => write!(f, "Classical({:?})", ct),
            FusionType::Tensor(tt) => {
                write!(f, "Tensor(shape={:?}, device={})", tt.shape, tt.device)
            }
            FusionType::Quantum(qt) => write!(f, "Quantum(qubits={})", qt.num_qubits),
            FusionType::Transitioning { from, to } => {
                write!(f, "Transitioning({:?} -> {:?})", from.type_hint(), to)
            }
        }
    }
}

/// Trait for types that can be converted to FusionType
pub trait IntoFusionType {
    fn into_fusion_type(self) -> FusionType;
}

impl IntoFusionType for i64 {
    fn into_fusion_type(self) -> FusionType {
        FusionType::int(self)
    }
}

impl IntoFusionType for f64 {
    fn into_fusion_type(self) -> FusionType {
        FusionType::float(self)
    }
}

impl IntoFusionType for bool {
    fn into_fusion_type(self) -> FusionType {
        FusionType::bool(self)
    }
}

impl IntoFusionType for String {
    fn into_fusion_type(self) -> FusionType {
        FusionType::string(self)
    }
}

impl IntoFusionType for &str {
    fn into_fusion_type(self) -> FusionType {
        FusionType::string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classical_types() {
        let int_val = FusionType::int(42);
        assert!(matches!(
            int_val,
            FusionType::Classical(ClassicalType::Int(42))
        ));

        let float_val = FusionType::float(3.14);
        assert!(matches!(
            float_val,
            FusionType::Classical(ClassicalType::Float(_))
        ));

        let bool_val = FusionType::bool(true);
        assert!(matches!(
            bool_val,
            FusionType::Classical(ClassicalType::Bool(true))
        ));
    }

    #[test]
    fn test_type_hints() {
        let classical = FusionType::int(10);
        assert!(matches!(classical.type_hint(), TypeHint::Classical));

        let tensor = FusionType::Tensor(TensorType {
            shape: vec![2, 3],
            dtype: TensorDType::F32,
            device: "cuda:0".to_string(),
            data_ptr: 0,
        });
        assert!(matches!(tensor.type_hint(), TypeHint::Tensor));
    }
}
