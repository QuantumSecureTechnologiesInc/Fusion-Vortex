//! Unitary trait for quantum gates
//! Integrated from fusion_core

use num_complex::Complex64;

/// Unitary trait for quantum operations
pub trait Unitary: Clone {
    /// Get the unitary matrix representation
    fn matrix(&self) -> Vec<Vec<Complex64>>;

    /// Get the adjoint (conjugate transpose)
    fn adjoint(&self) -> Self;

    /// Get the number of qubits this gate acts on
    fn num_qubits(&self) -> usize;

    /// Check if this is a Hermitian operator (self-adjoint)
    fn is_hermitian(&self) -> bool {
        // Default implementation - compare with adjoint
        // Actual implementation would compare matrices
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestGate;

    impl Unitary for TestGate {
        fn matrix(&self) -> Vec<Vec<Complex64>> {
            vec![
                vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
                vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
            ]
        }

        fn adjoint(&self) -> Self {
            Self
        }

        fn num_qubits(&self) -> usize {
            1
        }
    }

    #[test]
    fn test_unitary() {
        let gate = TestGate;
        assert_eq!(gate.num_qubits(), 1);
        let matrix = gate.matrix();
        assert_eq!(matrix.len(), 2);
    }
}
