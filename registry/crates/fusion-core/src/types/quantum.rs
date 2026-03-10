use super::tensor::Matrix;

#[derive(Debug, Clone, PartialEq)]
pub enum QuantumState {
    Real(Vec<u8>),
    Simulated(Vec<f64>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumGate {
    pub name: String,
    pub matrix: Matrix<f64>,
}

impl QuantumGate {
    pub fn new(name: impl Into<String>, matrix: Matrix<f64>) -> Self {
        Self {
            name: name.into(),
            matrix,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub operations: Vec<(QuantumGate, Vec<usize>)>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            operations: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, gate: QuantumGate, targets: Vec<usize>) {
        self.operations.push((gate, targets));
    }
}
