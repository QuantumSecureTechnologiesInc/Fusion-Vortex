//! Quantum registry for qubit management
//! Integrated from fusion_core Quantum Core.rs

use crate::simulator::QuantumState;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Unique identifier for a qubit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitId(pub usize);

/// Global registry for quantum simulator
/// Manages qubit states and enforces physical laws (no-cloning)
#[derive(Default)]
pub struct QuantumRegistry {
    /// Map Qubit ID -> Reference to the entangled State Vector it belongs to
    qubit_map: HashMap<QubitId, Arc<RwLock<QuantumState>>>,
    next_id: usize,
}

impl QuantumRegistry {
    /// Create new quantum registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate a new qubit in |0⟩ state
    pub fn allocate(&mut self) -> QubitId {
        let id = QubitId(self.next_id);
        self.next_id += 1;

        // New qubits start in |0⟩ state, independent (not entangled)
        let state = Arc::new(RwLock::new(QuantumState::zeros(1)));
        self.qubit_map.insert(id, state);
        id
    }

    /// Get the state associated with a qubit
    pub fn get_state(&self, id: QubitId) -> Option<Arc<RwLock<QuantumState>>> {
        self.qubit_map.get(&id).cloned()
    }
}

/// Physical Qubit Handle
/// Enforces No-Cloning via Rust's Move semantics
#[derive(Debug)]
pub struct Qubit {
    pub id: QubitId,
}

impl Qubit {
    /// Create a new qubit (allocates from registry)
    pub fn new(registry: &mut QuantumRegistry) -> Self {
        let id = registry.allocate();
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qubit_allocation() {
        let mut registry = QuantumRegistry::new();
        let q1 = Qubit::new(&mut registry);
        let q2 = Qubit::new(&mut registry);

        assert_ne!(q1.id, q2.id);
    }

    #[test]
    fn test_registry_state() {
        let mut registry = QuantumRegistry::new();
        let q = Qubit::new(&mut registry);

        let state = registry.get_state(q.id);
        assert!(state.is_some());
    }
}
