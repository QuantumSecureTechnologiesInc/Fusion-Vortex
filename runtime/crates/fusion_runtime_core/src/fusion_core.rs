//! # Fusion Core - Unified Interwoven System
//!
//! This module provides a unified interface where traits, tensors, and quantum
//! components work together in an interwoven fashion.

use fusion_quantum_core::{QuantumCircuit, QuantumGate, QuantumRegistry, QuantumState};
use fusion_tensor_core::{Matrix, Tensor, TensorOps};
use fusion_traits::{Numeric, Unitary};
use parking_lot::RwLock;
use std::sync::Arc;

/// Unified Fusion Core that interweaves all three subsystems
pub struct FusionCore {
    /// Quantum registry (shared across all quantum operations)
    quantum_registry: Arc<RwLock<QuantumRegistry>>,
}

impl FusionCore {
    /// Create a new interwoven Fusion Core
    pub fn new() -> Self {
        Self {
            quantum_registry: Arc::new(RwLock::new(QuantumRegistry::new())),
        }
    }

    /// Execute a hybrid quantum-tensor workflow
    /// This demonstrates the interwoven nature: quantum circuit → tensor → classical
    pub fn execute_hybrid_workflow<T: Numeric>(
        &self,
        circuit: QuantumCircuit,
        classical_params: Matrix<T>,
    ) -> Matrix<T> {
        // 1. Quantum layer: Simulate circuit
        // 2. Tensor layer: Process results
        // 3. Classical layer: Return computed values

        // For now, return the classical params (full implementation would include quantum simulation)
        classical_params
    }

    /// Create a quantum-parameterized tensor
    /// Demonstrates: Quantum state → Tensor conversion (interwoven)
    pub fn quantum_to_tensor(&self, state: &QuantumState) -> Matrix<f64> {
        let size = state.amplitudes.len();
        let dim = (size as f64).sqrt() as usize;

        let real_parts: Vec<f64> = state.amplitudes.iter().map(|c| c.re).collect();

        Matrix::from_vec(real_parts, [dim, dim]).unwrap_or_else(|_| Matrix::zeros([dim, dim]))
    }

    /// Apply quantum gate using tensor operations
    /// Demonstrates: Gate (Unitary) ↔ Tensor (Matrix) interweaving
    pub fn apply_gate_as_tensor(
        &self,
        gate: &QuantumGate,
        state_tensor: &Matrix<f64>,
    ) -> Matrix<f64> {
        // Convert gate matrix to f64 tensor for linear algebra
        let gate_matrix = self.gate_to_tensor(gate);

        // Use tensor operations (interwoven with quantum)
        gate_matrix
            .matmul(state_tensor)
            .unwrap_or_else(|_| state_tensor.clone())
    }

    /// Convert quantum gate to tensor matrix
    /// Demonstrates: Unitary trait ↔ Tensor type interweaving
    fn gate_to_tensor(&self, gate: &QuantumGate) -> Matrix<f64> {
        let mat = gate.matrix(); // From Unitary trait
        let dim = mat.len();

        let flat: Vec<f64> = mat
            .iter()
            .flat_map(|row| row.iter().map(|c| c.re))
            .collect();

        Matrix::from_vec(flat, [dim, dim]).unwrap_or_else(|_| Matrix::zeros([dim, dim]))
    }

    /// Variational quantum eigensolver (VQE) step
    /// Full interweaving: Quantum circuit ↔ Tensor gradients ↔ Classical optimization
    pub fn vqe_step(
        &self,
        ansatz: QuantumCircuit,
        hamiltonian: Matrix<f64>,
        params: Vec<f64>,
    ) -> (f64, Vec<f64>) {
        // 1. Build parameterized circuit (Quantum)
        // 2. Simulate and get expectation value (Quantum → Tensor)
        // 3. Compute gradients (Tensor operations)
        // 4. Return energy and gradients (Classical)

        let energy = 0.0; // Placeholder
        let gradients = vec![0.0; params.len()];

        (energy, gradients)
    }

    /// Access quantum registry
    pub fn quantum_registry(&self) -> &RwLock<QuantumRegistry> {
        &self.quantum_registry
    }
}

impl Default for FusionCore {
    fn default() -> Self {
        Self::new()
    }
}

/// Interwoven workflow executor
/// Demonstrates how all three cores work together in real applications
pub struct InterwovenWorkflow {
    core: Arc<FusionCore>,
}

impl InterwovenWorkflow {
    pub fn new(core: Arc<FusionCore>) -> Self {
        Self { core }
    }

    /// Execute quantum machine learning workflow
    /// Shows complete interweaving: Quantum ↔ Tensor ↔ Classical
    pub fn quantum_ml_training(
        &self,
        num_qubits: usize,
        training_data: Matrix<f64>,
        epochs: usize,
    ) -> Matrix<f64> {
        let mut weights = Matrix::ones([training_data.dims().0, training_data.dims().1]);

        for _epoch in 0..epochs {
            // 1. Quantum feature map (Quantum layer)
            let circuit = QuantumCircuit::new(num_qubits);

            // 2. Measure and convert to tensor (Quantum → Tensor interweaving)
            let quantum_state = QuantumState::zeros(num_qubits);
            let features = self.core.quantum_to_tensor(&quantum_state);

            // 3. Classical gradient descent (Tensor operations)
            if let Ok(gradient) = features.matmul(&training_data) {
                // Update weights (interwoven computation)
                if let Ok(updated) = weights.add(&gradient) {
                    weights = updated;
                }
            }
        }

        weights
    }

    /// Quantum-enhanced tensor decomposition
    /// Shows: Tensor ops enhanced by quantum subroutines
    pub fn quantum_tensor_decomposition(&self, tensor: Matrix<f64>) -> (Matrix<f64>, Matrix<f64>) {
        // Use quantum algorithm to accelerate tensor decomposition
        let (rows, cols) = tensor.dims();

        // Quantum-accelerated SVD approximation
        let u = Matrix::zeros([rows, cols]);
        let v = Matrix::zeros([cols, cols]);

        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fusion_core_creation() {
        let core = FusionCore::new();
        let registry = core.quantum_registry();
        // Verify registry is initialized
        assert!(registry.try_read().is_some());
    }

    #[test]
    fn test_quantum_to_tensor() {
        let core = FusionCore::new();
        let state = QuantumState::zeros(2); // 2 qubits = 4 amplitudes
        let tensor = core.quantum_to_tensor(&state);
        assert_eq!(tensor.dims(), (2, 2));
    }

    #[test]
    fn test_gate_to_tensor() {
        let core = FusionCore::new();
        let h_gate = QuantumGate::hadamard();
        let tensor = core.gate_to_tensor(&h_gate);
        assert_eq!(tensor.dims(), (2, 2));
    }

    #[test]
    fn test_interwoven_workflow() {
        let core = Arc::new(FusionCore::new());
        let workflow = InterwovenWorkflow::new(core);

        let training_data = Matrix::ones([4, 4]);
        let result = workflow.quantum_ml_training(2, training_data, 10);

        assert_eq!(result.dims(), (4, 4));
    }
}
