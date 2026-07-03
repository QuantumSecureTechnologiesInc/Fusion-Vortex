//! Oscillating Security Mesh - Moving Target Defence (MTD)
//!
//! Dynamic access vector system that shifts periodically to prevent static attack surfaces.

use crate::chaos::ChaosEngine;
use std::time::{SystemTime, UNIX_EPOCH};

/// Security state of the mesh
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SecurityState {
    /// System is secure and accepting requests
    Secure,
    /// Security vector is currently shifting
    Shifting,
    /// System is locked and rejecting all requests
    Locked,
}

/// Oscillating Security Mesh providing Moving Target Defence
pub struct OscillatingMesh {
    chaos: ChaosEngine,
    current_vector: u32,
    last_shift: u64,
    shift_interval: u64,
    state: SecurityState,
}

impl OscillatingMesh {
    /// Create a new Oscillating Mesh with default parameters
    pub fn new() -> Self {
        let mut mesh = OscillatingMesh {
            chaos: ChaosEngine::new(0.314159),
            current_vector: 0,
            last_shift: 0,
            shift_interval: 5, // Slower oscillation for stability in production
            state: SecurityState::Secure,
        };
        mesh.shift_vector();
        mesh
    }

    /// Get current time in seconds since UNIX epoch
    fn get_time(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Shift the security vector to a new value
    fn shift_vector(&mut self) {
        let entropy = self.chaos.next_val();
        // Dynamic vector range: 8000 - 9000
        self.current_vector = 8000 + (entropy * 1000.0) as u32;
        self.last_shift = self.get_time();
        tracing::debug!("Mesh Shifted. New Entry Vector: {}", self.current_vector);
    }

    /// Get the valid vector, shifting if necessary
    pub fn get_valid_vector(&mut self) -> u32 {
        if self.get_time() - self.last_shift > self.shift_interval {
            self.shift_vector();
            self.state = SecurityState::Shifting;
        } else {
            self.state = SecurityState::Secure;
        }
        self.current_vector
    }

    /// Validate an access attempt against the current vector
    ///
    /// # Arguments
    ///
    /// * `vector_attempt` - The vector value to validate
    ///
    /// # Returns
    ///
    /// `true` if the vector matches the current valid vector, `false` otherwise
    pub fn validate_access(&mut self, vector_attempt: u32) -> bool {
        let target = self.get_valid_vector();
        // Production MTD demands exactness
        vector_attempt == target
    }

    /// Get the current security state
    pub fn state(&self) -> SecurityState {
        self.state
    }
}

impl Default for OscillatingMesh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_creation() {
        let mesh = OscillatingMesh::new();
        assert!(mesh.current_vector >= 8000 && mesh.current_vector <= 9000);
    }

    #[test]
    fn test_vector_validation() {
        let mut mesh = OscillatingMesh::new();
        let valid_vector = mesh.get_valid_vector();

        assert!(mesh.validate_access(valid_vector));
        assert!(!mesh.validate_access(valid_vector + 1));
    }

    #[test]
    fn test_security_states() {
        let mesh = OscillatingMesh::new();
        assert!(matches!(
            mesh.state(),
            SecurityState::Secure | SecurityState::Shifting
        ));
    }
}
