# Fusion Gate Decomposition

**Version:** 0.2.0  
**Type:** Quantum Compiler Pass  
**License:** MIT

## Overview

Fusion Gate Decomposition (`fusion_gate_decomposition`) is a library for breaking down high-level quantum gates (like Toffoli, Fredkin, or arbitrary Unitaries) into hardware-native basic gates (like CX, U3, RZ).

## Features

- **Universal Decomposition**: Decomposes any unitary matrix to basic gates (Solovay-Kitaev)
- **Optimization**: Finds optimal decomposition to minimize circuit depth
- **Hardware Aware**: Supports different target gate sets (IBM, Rigetti, IonQ)

## Usage

```rust
use fusion_gate_decomposition::Decomposer;
use fusion_quantum_sdk::Gate;

let decomposer = Decomposer::new().target_set(&["cx", "u3"]);
let dense_gate = Gate::Toffoli(0, 1, 2);

let circuit = decomposer.decompose(&dense_gate)?;
// Result: Sequence of H, T, CX, T_dagger...
```

## Dependencies

- `fusion_core`
- `fusion_quantum_sdk`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
