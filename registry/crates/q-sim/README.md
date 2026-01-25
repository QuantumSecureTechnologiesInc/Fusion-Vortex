# Quantum Simulator

A high-performance state-vector simulator for Fusion quantum circuits.

## Features

- Exact state vector simulation
- GPU acceleration support (optional)
- Noise simulation capabilities

## Usage

```rust
use q_sim::Simulator;

let result = Simulator::run(&circuit)?;
```text