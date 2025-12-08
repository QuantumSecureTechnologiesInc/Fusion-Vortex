// src/quantum/circuit.rs - Quantum Circuit Builder
// Constructs and manages quantum circuits

use super::gates::Gate;
use super::{Complex, QuantumError, QuantumStats, StateVector};

/// Quantum circuit
pub struct QuantumCircuit {
    /// Number of qubits
    num_qubits: usize,
    /// Gates in the circuit
    pub(crate) gates: Vec<CircuitGate>,
    /// Circuit name
    name: String,
    /// Statistics
    stats: QuantumStats,
}

/// Gate applied to circuit
#[derive(Debug, Clone)]
pub struct CircuitGate {
    pub gate: Gate,
    pub target_qubits: Vec<usize>,
    pub control_qubits: Vec<usize>,
}

impl QuantumCircuit {
    /// Create a new quantum circuit
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
            name: format!("Circuit_{}_qubits", num_qubits),
            stats: QuantumStats::default(),
        }
    }

    /// Create named circuit
    pub fn with_name(num_qubits: usize, name: impl Into<String>) -> Self {
        let mut circuit = Self::new(num_qubits);
        circuit.name = name.into();
        circuit.stats.circuits_created += 1;
        circuit
    }

    /// Get number of qubits
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get number of gates
    pub fn num_gates(&self) -> usize {
        self.gates.len()
    }

    /// Apply a gate to target qubits
    pub fn apply_gate(&mut self, gate: Gate, targets: Vec<usize>) -> Result<(), QuantumError> {
        // Validate qubit indices
        for &qubit in &targets {
            if qubit >= self.num_qubits {
                return Err(QuantumError::InvalidQubit(qubit));
            }
        }

        self.gates.push(CircuitGate {
            gate,
            target_qubits: targets,
            control_qubits: Vec::new(),
        });

        self.stats.gates_applied += 1;
        Ok(())
    }

    /// Apply controlled gate
    pub fn apply_controlled_gate(
        &mut self,
        gate: Gate,
        control: usize,
        target: usize,
    ) -> Result<(), QuantumError> {
        if control >= self.num_qubits {
            return Err(QuantumError::InvalidQubit(control));
        }
        if target >= self.num_qubits {
            return Err(QuantumError::InvalidQubit(target));
        }

        self.gates.push(CircuitGate {
            gate,
            target_qubits: vec![target],
            control_qubits: vec![control],
        });

        self.stats.gates_applied += 1;
        Ok(())
    }

    /// Common gates with helper methods

    /// Apply Hadamard gate
    pub fn h(&mut self, qubit: usize) -> Result<(), QuantumError> {
        self.apply_gate(Gate::Hadamard, vec![qubit])
    }

    /// Apply Pauli-X gate
    pub fn x(&mut self, qubit: usize) -> Result<(), QuantumError> {
        self.apply_gate(Gate::PauliX, vec![qubit])
    }

    /// Apply Pauli-Y gate
    pub fn y(&mut self, qubit: usize) -> Result<(), QuantumError> {
        self.apply_gate(Gate::PauliY, vec![qubit])
    }

    /// Apply Pauli-Z gate
    pub fn z(&mut self, qubit: usize) -> Result<(), QuantumError> {
        self.apply_gate(Gate::PauliZ, vec![qubit])
    }

    /// Apply CNOT gate
    pub fn cnot(&mut self, control: usize, target: usize) -> Result<(), QuantumError> {
        self.apply_controlled_gate(Gate::PauliX, control, target)
    }

    /// Apply rotation around X axis
    pub fn rx(&mut self, qubit: usize, angle: f64) -> Result<(), QuantumError> {
        self.apply_gate(Gate::RotationX(angle), vec![qubit])
    }

    /// Apply rotation around Y axis
    pub fn ry(&mut self, qubit: usize, angle: f64) -> Result<(), QuantumError> {
        self.apply_gate(Gate::RotationY(angle), vec![qubit])
    }

    /// Apply rotation around Z axis
    pub fn rz(&mut self, qubit: usize, angle: f64) -> Result<(), QuantumError> {
        self.apply_gate(Gate::RotationZ(angle), vec![qubit])
    }

    /// Get initial state vector |00...0⟩
    pub fn initial_state(&self) -> StateVector {
        let size = 1 << self.num_qubits; // 2^n
        let mut state = vec![Complex::zero(); size];
        state[0] = Complex::one(); // |0...0⟩ = 1, all others = 0
        state
    }

    /// Get circuit depth (longest path)
    pub fn depth(&self) -> usize {
        // Simplified - would calculate actual depth considering parallelism
        self.gates.len()
    }

    /// Print circuit diagram
    pub fn print_diagram(&self) {
        println!("\nQuantum Circuit: {}", self.name);
        println!("Qubits: {} | Gates: {}", self.num_qubits, self.gates.len());
        println!("{}", "=".repeat(50));

        for (i, circuit_gate) in self.gates.iter().enumerate() {
            if circuit_gate.control_qubits.is_empty() {
                println!(
                    "Step {}: {:?} on qubits {:?}",
                    i + 1,
                    circuit_gate.gate,
                    circuit_gate.target_qubits
                );
            } else {
                println!(
                    "Step {}: {:?} on qubits {:?} (controlled by {:?})",
                    i + 1,
                    circuit_gate.gate,
                    circuit_gate.target_qubits,
                    circuit_gate.control_qubits
                );
            }
        }
        println!("{}", "=".repeat(50));
    }

    /// Get statistics
    pub fn stats(&self) -> &QuantumStats {
        &self.stats
    }
}

/// Circuit builder for common patterns
pub struct CircuitBuilder;

impl CircuitBuilder {
    /// Create a new empty circuit
    pub fn new(num_qubits: usize) -> QuantumCircuit {
        QuantumCircuit::new(num_qubits)
    }

    /// Bell state circuit: creates entangled pair
    pub fn bell_state() -> QuantumCircuit {
        let mut circuit = QuantumCircuit::with_name(2, "Bell State");
        circuit.h(0).unwrap();
        circuit.cnot(0, 1).unwrap();
        circuit
    }

    /// Quantum Fourier Transform
    pub fn qft(num_qubits: usize) -> QuantumCircuit {
        let mut circuit = QuantumCircuit::with_name(num_qubits, "QFT");
        // Simplified QFT implementation
        for i in 0..num_qubits {
            circuit.h(i).unwrap();
        }
        circuit
    }

    /// Grover's search circuit
    pub fn grover_search(num_qubits: usize) -> QuantumCircuit {
        let mut circuit = QuantumCircuit::with_name(num_qubits, "Grover Search");
        // Initialize in superposition
        for i in 0..num_qubits {
            circuit.h(i).unwrap();
        }
        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_creation() {
        let circuit = QuantumCircuit::new(3);
        assert_eq!(circuit.num_qubits(), 3);
        assert_eq!(circuit.num_gates(), 0);
    }

    #[test]
    fn test_apply_hadamard() {
        let mut circuit = QuantumCircuit::new(2);
        assert!(circuit.h(0).is_ok());
        assert_eq!(circuit.num_gates(), 1);
    }

    #[test]
    fn test_invalid_qubit() {
        let mut circuit = QuantumCircuit::new(2);
        assert!(circuit.h(5).is_err());
    }

    #[test]
    fn test_cnot() {
        let mut circuit = QuantumCircuit::new(2);
        assert!(circuit.cnot(0, 1).is_ok());
        assert_eq!(circuit.num_gates(), 1);
    }

    #[test]
    fn test_bell_state() {
        let circuit = CircuitBuilder::bell_state();
        assert_eq!(circuit.num_qubits(), 2);
        assert_eq!(circuit.num_gates(), 2); // H + CNOT
    }

    #[test]
    fn test_initial_state() {
        let circuit = QuantumCircuit::new(2);
        let state = circuit.initial_state();

        assert_eq!(state.len(), 4); // 2^2
        assert_eq!(state[0].real, 1.0);
        assert_eq!(state[1].real, 0.0);
    }
}
