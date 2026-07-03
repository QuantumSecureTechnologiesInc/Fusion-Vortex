//! Error types for quantum operations

use std::fmt;

#[derive(Debug, Clone)]
pub enum QuantumError {
    GateArityMismatch {
        gate: String,
        required: usize,
        provided: usize,
    },
    InvalidQubitAccess(usize),
}

impl fmt::Display for QuantumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GateArityMismatch {
                gate,
                required,
                provided,
            } => {
                write!(
                    f,
                    "Gate {} requires {} qubits, got {}",
                    gate, required, provided
                )
            }
            Self::InvalidQubitAccess(id) => {
                write!(f, "Invalid qubit access: qubit {} does not exist", id)
            }
        }
    }
}

impl std::error::Error for QuantumError {}

pub type QuantumResult<T> = Result<T, QuantumError>;
