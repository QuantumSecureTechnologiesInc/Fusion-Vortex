//! Quantum circuit construction
//! Integrated from fusion_core Quantum Core.rs

use crate::error::{QuantumError, QuantumResult};
use crate::gates::QuantumGate;

/// Quantum circuit
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<(QuantumGate, Vec<usize>)>,
}

impl QuantumCircuit {
    /// Create a new quantum circuit with specified number of qubits
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
        }
    }

    /// Apply a gate to specified qubits
    pub fn apply_gate(&mut self, gate: QuantumGate, targets: Vec<usize>) -> QuantumResult<()> {
        // Validate gate arity
        if gate.num_qubits != targets.len() {
            return Err(QuantumError::GateArityMismatch {
                gate: gate.name.clone(),
                required: gate.num_qubits,
                provided: targets.len(),
            });
        }

        // Validate qubit indices
        for &t in &targets {
            if t >= self.num_qubits {
                return Err(QuantumError::InvalidQubitAccess(t));
            }
        }

        self.gates.push((gate, targets));
        Ok(())
    }

    /// Get the number of gates in the circuit
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }

    /// Get circuit depth (number of layers)
    pub fn depth(&self) -> usize {
        // Simplified: just return gate count
        // Real impl would compute parallel layers
        self.gates.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gates::QuantumGate;
    use fusion_tensor_core::Matrix;
    use num_complex::Complex64;

    #[test]
    fn test_circuit_creation() {
        let circuit = QuantumCircuit::new(2);
        assert_eq!(circuit.num_qubits, 2);
        assert_eq!(circuit.gate_count(), 0);
    }

    #[test]
    fn test_apply_gate() {
        let mut circuit = QuantumCircuit::new(2);

        let h_gate = QuantumGate {
            name: "H".to_string(),
            matrix: Matrix::from_vec(
                vec![
                    Complex64::new(1.0, 0.0),
                    Complex64::new(1.0, 0.0),
                    Complex64::new(1.0, 0.0),
                    Complex64::new(-1.0, 0.0),
                ],
                [2, 2],
            )
            .unwrap(),
            num_qubits: 1,
        };

        let result = circuit.apply_gate(h_gate, vec![0]);
        assert!(result.is_ok());
        assert_eq!(circuit.gate_count(), 1);
    }

    #[test]
    fn test_invalid_qubit() {
        let mut circuit = QuantumCircuit::new(2);

        let h_gate = QuantumGate {
            name: "H".to_string(),
            matrix: Matrix::zeros([2, 2]),
            num_qubits: 1,
        };

        let result = circuit.apply_gate(h_gate, vec![5]);
        assert!(result.is_err());
    }
}
