use crate::types::tensor::Matrix;
use num_complex::Complex64;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qubit {
    pub id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumState {
    Simulated(Vec<Complex64>),
    Real(Vec<u8>), // Measurement data
}

// No-Cloning: Clone is NOT derived for Qubit.

#[derive(Clone, Debug)]
pub struct QuantumGate {
    pub name: String,
    pub matrix: Matrix<Complex64>,
}

impl QuantumGate {
    pub fn new(name: &str, matrix: Matrix<Complex64>) -> Self {
        Self {
            name: name.to_string(),
            matrix,
        }
    }

    pub fn hadamard() -> Self {
        // H = 1/sqrt(2) * [[1, 1], [1, -1]]
        let s = 1.0 / 2.0f64.sqrt();
        let data = vec![
            Complex64::new(s, 0.0),
            Complex64::new(s, 0.0),
            Complex64::new(s, 0.0),
            Complex64::new(-s, 0.0),
        ];
        let matrix = Matrix::new(data, [2, 2]).unwrap();
        Self::new("H", matrix)
    }

    pub fn cnot() -> Self {
        // CNOT 4x4
        let zero = Complex64::new(0.0, 0.0);
        let one = Complex64::new(1.0, 0.0);
        let data = vec![
            one, zero, zero, zero, zero, one, zero, zero, zero, zero, zero, one, zero, zero, one,
            zero,
        ];
        let matrix = Matrix::new(data, [4, 4]).unwrap();
        Self::new("CNOT", matrix)
    }
}

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

    pub fn apply_gate(&mut self, gate: QuantumGate, targets: Vec<usize>) {
        self.operations.push((gate, targets));
    }

    pub fn simulate(&self) -> Vec<Complex64> {
        // Placeholder simulation: return a state vector of size 2^n
        // Initial state |0...0>
        let size = 1 << self.num_qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        if size > 0 {
            state[0] = Complex64::new(1.0, 0.0);
        }

        // TODO: Full simulation using gate matrices

        state
    }
}
