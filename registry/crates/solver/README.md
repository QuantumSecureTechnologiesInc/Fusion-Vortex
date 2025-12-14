# Fusion Solver

**Version:** 0.2.0  
**Type:** Mathematical Solver  
**License:** MIT

## Overview

Fusion Solver (`fusion_solver`) is a high-performance generic constraint compliance and equation solving engine. It is used for resolving dependencies, optimizing resource allocation, and solving symbolic math problems.

## Features

- **Constraint Solving**: SAT/SMT-style solver interface
- **Symbolic Math**: Algebraic simplification and solving
- **Optimization**: Gradient-based and gradient-free optimizers
- **Integration**: Works directly with `fusion_ai_core` tensors

## Usage

```rust
use fusion_solver::{Solver, Constraint};

let mut solver = Solver::new();
solver.add(Constraint::eq("x + y", 10));
solver.add(Constraint::gt("x", 5));

let solution = solver.solve()?;
println!("x = {}, y = {}", solution["x"], solution["y"]);
```

## Dependencies

- `fusion_core`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
