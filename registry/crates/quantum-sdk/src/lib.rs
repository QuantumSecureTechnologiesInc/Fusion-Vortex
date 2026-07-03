/// Fusion Quantum SDK
/// Provides interfaces for Quantum Processing Units (QPUs).
use fusion_core::types::quantum::QuantumCircuit;

pub trait OptimizationPass {
    fn run(&self, circuit: &mut QuantumCircuit);
}

pub struct QuantumBackend;
pub struct CircuitCompiler;
