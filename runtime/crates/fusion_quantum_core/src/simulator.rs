//! Quantum state vector simulator
//! Integrated from fusion_core Quantum Core.rs

use num_complex::Complex64;

/// Quantum state vector
#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitudes: Vec<Complex64>,
    pub num_qubits: usize,
}

impl QuantumState {
    /// Create state in |0...0⟩
    pub fn zeros(num_qubits: usize) -> Self {
        let size = 1 << num_qubits; // 2^num_qubits
        let mut amplitudes = vec![Complex64::new(0.0, 0.0); size];
        amplitudes[0] = Complex64::new(1.0, 0.0); // |0⟩ state

        Self {
            amplitudes,
            num_qubits,
        }
    }

    /// Get probability of measuring |i⟩
    pub fn probability(&self, state_index: usize) -> f64 {
        if state_index < self.amplitudes.len() {
            self.amplitudes[state_index].norm_sqr()
        } else {
            0.0
        }
    }

    /// Get total probability (should be 1.0 for normalized states)
    pub fn total_probability(&self) -> f64 {
        self.amplitudes.iter().map(|a| a.norm_sqr()).sum()
    }

    /// Normalize the state vector
    pub fn normalize(&mut self) {
        let norm = self.total_probability().sqrt();
        if norm > 1e-10 {
            for amp in &mut self.amplitudes {
                *amp /= norm;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let state = QuantumState::zeros(2);
        assert_eq!(state.num_qubits, 2);
        assert_eq!(state.amplitudes.len(), 4);
        assert_eq!(state.probability(0), 1.0);
    }

    #[test]
    fn test_normalization() {
        let mut state = QuantumState {
            amplitudes: vec![Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)],
            num_qubits: 1,
        };

        state.normalize();
        let total = state.total_probability();
        assert!((total - 1.0).abs() < 1e-10);
    }
}
