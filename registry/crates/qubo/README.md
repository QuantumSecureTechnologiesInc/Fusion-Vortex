# QUBO Solver

Quadratic Unconstrained Binary Optimization (QUBO) tools for Fusion.

## Features
- Problem formulation
- Simulated annealing solver
- Interface to external quantum annealers

## Usage
```rust
use qubo::Problem;

let mut problem = Problem::new();
problem.add_term(1, 2, 0.5);
```
