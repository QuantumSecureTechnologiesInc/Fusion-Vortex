# Hybrid Quantum Optimizer

Optimizers for hybrid quantum-classical algorithms (like VQE, QAOA).

## Features
- SPSA, COBYLA, L-BFGS-B implementations
- Gradient descent for parameterized circuits
- Noise-resilient optimization strategies

## Usage
```rust
use q_optimizer_hybrid::SPSA;

let optimizer = SPSA::new().max_iter(100);
```
