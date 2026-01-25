# Quantum Algorithms

A collection of standard quantum algorithms implemented in Fusion.

## Features

- Shor's algorithm
- Grover's algorithm
- Quantum Phase Estimation (QPE)
- QAOA

## Usage

```rust
use q_algo::grover;

let result = grover::search(oracle, n_qubits);
```text