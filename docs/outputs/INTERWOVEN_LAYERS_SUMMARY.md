# Interwoven Neural Network Layers Implementation

**Date**: 2025-12-08
**Status**: Implemented

## Overview
As per the core philosophy of **Fusion**, we have implemented neural network layers that are **Interwoven** rather than strictly layered. This demonstrates the seamless integration of Classical and Quantum domains within a single module.

## Implemented Layers (`src/ml/nn/layers.rs`)

### 1. Classical `Linear` Layer
*   Standard Dense layer operation ($y = xW + b$).
*   Implemented `Module` trait for forward pass, parameter management, and training mode.
*   Uses `Tensor` operations optimized for Fusion.

### 2. Interwoven `HybridQuantumLayer`
This layer purely embodies the **Interwoven** philosophy:
*   **Input**: Classical Tensor (e.g., from a Linear layer).
*   **Process**:
    1.  **Angle Encoding**: Maps classical values to Quantum States (e.g., rotation gates).
    2.  **Variational Circuit**: Applies parameterized quantum gates (entanglement + rotation).
    3.  **Simulation**: seamlessly calls the `QuantumSimulator` engine.
    4.  **Measurement**: Collapses state back to Classical bits.
*   **Output**: Classical Tensor (ready for next NN layer).

This proves that in Fusion, "AI" and "Quantum" are not separate libraries but fluidly connected capabilities.

## Code Structure
```rust
pub struct HybridQuantumLayer {
    params: Tensor,              // Classical Learnable Params
    simulator: QuantumSimulator, // Quantum Engine
}

impl Module for HybridQuantumLayer {
    fn forward(&self, input: &Tensor) -> Result<Tensor, MLError> {
        // Interweaving: Classical Data -> Quantum Gate Parameters
        // ...
        // Execution: Quantum Simulation
        // ...
        // Return: Classical Data
    }
}
```

## Verification
*   Added unit tests in `src/ml/nn/layers.rs`:
    *   `test_linear_forward`: Validates classical dense connectivity.
    *   `test_hybrid_quantum_layer`: Validates the full interwoven pipeline (Classical -> Quantum -> Classical).
