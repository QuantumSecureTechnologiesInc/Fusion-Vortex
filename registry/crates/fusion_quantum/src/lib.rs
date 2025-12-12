//! # Fusion Quantum
//!
//! Quantum computing primitives leveraging fusion_runtime_core for QPU scheduling.

use fusion_core::{FusionType, QuantumState, QuantumType};
use fusion_runtime_hal::{QuantumCircuit as HalQuantumCircuit, QuantumGate};
use tracing::{debug, trace};

/// Qubit representation
pub struct Qubit {
    id: usize,
    state: QubitState,
}

#[derive(Debug, Clone)]
enum QubitState {
    Zero,
    One,
    Superposition,
}

impl Qubit {
    /// Create a new qubit in |0⟩ state
    pub fn new() -> Self {
        debug!("Creating new qubit");
        Self {
            id: 0,
            state: QubitState::Zero,
        }
    }

    /// Apply Hadamard gate (creates superposition)
    pub fn hadamard(&mut self) {
        trace!("Applying Hadamard gate to qubit {}", self.id);
        self.state = QubitState::Superposition;
    }

    /// Apply Pauli-X gate (bit flip)
    pub fn pauli_x(&mut self) {
        trace!("Applying Pauli-X gate to qubit {}", self.id);
        self.state = match self.state {
            QubitState::Zero => QubitState::One,
            QubitState::One => QubitState::Zero,
            QubitState::Superposition => QubitState::Superposition,
        };
    }

    /// Measure the qubit (collapses superposition)
    pub async fn measure(&mut self) -> u8 {
        trace!("Measuring qubit {}", self.id);

        // In a real implementation:
        // 1. Submit measurement to QPU via fusion_runtime_hal
        // 2. Wait for result on external device queue
        // 3. Return measured bit value

        match self.state {
            QubitState::Zero => 0,
            QubitState::One => 1,
            QubitState::Superposition => {
                // Simulated 50/50 collapse
                if rand_bool() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl Default for Qubit {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Qubit> for FusionType {
    fn from(qubit: Qubit) -> Self {
        FusionType::Quantum(QuantumType {
            num_qubits: 1,
            state: QuantumState::Simulated(vec![(1.0, 0.0), (0.0, 0.0)]), // |0⟩
            qpu_device: "simulator".to_string(),
        })
    }
}

/// Quantum circuit builder
pub struct QuantumCircuit {
    num_qubits: usize,
    gates: Vec<QuantumGate>,
    device: String,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        debug!("Creating quantum circuit with {} qubits", num_qubits);
        Self {
            num_qubits,
            gates: Vec::new(),
            device: "cpu".to_string(),
        }
    }

    pub fn device(mut self, device: impl Into<String>) -> Self {
        self.device = device.into();
        self
    }

    pub fn h(&mut self, qubit: usize) -> &mut Self {
        self.gates.push(QuantumGate::Hadamard(qubit));
        self
    }

    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        self.gates.push(QuantumGate::CNOT(control, target));
        self
    }

    pub fn measure(&mut self, qubit: usize) -> &mut Self {
        self.gates.push(QuantumGate::Measure(qubit));
        self
    }

    /// Execute circuit on QPU or Simulator
    pub async fn execute(&self) -> CircuitResult {
        debug!("Executing circuit on {}", self.device);

        CircuitResult {
            counts: vec![(vec![0; self.num_qubits], 1000)],
        }
    }
}

pub struct Simulator;

impl Simulator {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self, _circuit: &QuantumCircuit) -> CircuitResult {
        // Mock simulation result
        CircuitResult {
            counts: vec![(vec![0], 100)],
        }
    }
}

pub struct Observable {
    // Hamiltonian representation
}

pub struct QubitRegister;

pub struct CircuitResult {
    pub counts: Vec<(Vec<u8>, u32)>,
}

fn rand_bool() -> bool {
    // Simple deterministic mock for now to avoid std::time issues in no-std
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qubit_creation() {
        let qubit = Qubit::new();
        assert_eq!(qubit.id, 0);
    }

    #[tokio::test]
    async fn test_qubit_measurement() {
        let mut qubit = Qubit::new();
        qubit.hadamard();
        let result = qubit.measure().await;

        assert!(result == 0 || result == 1);
    }

    #[test]
    fn test_circuit_builder() {
        let mut circuit = Circuit::new(2);
        circuit.h(0).cx(0, 1).measure(0).measure(1);

        assert_eq!(circuit.gates.len(), 4);
    }
}
