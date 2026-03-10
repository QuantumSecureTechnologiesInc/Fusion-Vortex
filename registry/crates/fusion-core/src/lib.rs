mod error;
pub mod types;

pub use error::{FusionError, FusionResult};
pub use types::quantum::{QuantumCircuit, QuantumGate, QuantumState};
pub use types::tensor::{Matrix, Tensor};
