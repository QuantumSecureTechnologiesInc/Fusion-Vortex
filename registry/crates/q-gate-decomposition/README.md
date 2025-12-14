# Quantum Gate Decomposition

Tools for decomposing complex quantum gates into native gate sets.

## Features
- Basis gate compilation
- KAK decomposition
- Euler angle decomposition

## Usage
```rust
use q_gate_decomposition::decompose;

let native_gates = decompose(unary_unitary, target_basis)?;
```
