# QAOA Implementation

A dedicated crate for the Quantum Approximate Optimization Algorithm (QAOA) in Fusion.

## Features

- Mixing and cost Hamiltonian generation
- Variational form construction
- Optimization loop integration

## Usage

```rust
use qaoa::QAOA;

let solver = QAOA::new(hamiltonian, p_steps);
```text