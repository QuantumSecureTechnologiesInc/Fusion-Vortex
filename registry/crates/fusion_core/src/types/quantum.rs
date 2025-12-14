use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self { num_qubits }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumState {
    Simulated(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumGate {
    H(usize),
    CNOT(usize, usize),
    X(usize),
    Y(usize),
    Z(usize),
    Rx(usize, f64),
    Ry(usize, f64),
    Rz(usize, f64),
}
