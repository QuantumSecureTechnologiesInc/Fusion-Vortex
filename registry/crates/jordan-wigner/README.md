# Fusion Jordan-Wigner

**Version:** 0.2.0  
**Type:** Quantum Transform  
**License:** MIT

## Overview

Fusion Jordan-Wigner (`fusion_jordan_wigner`) implements the Jordan-Wigner transformation, a critical algorithm for mapping fermionic systems (like electrons in chemistry) onto qubit systems for quantum simulation.

## Features

- **Fermion Mapping**: Maps creation/annihilation operators to Pauli strings
- **Efficiency**: Optimized handling of large Hamiltonian terms
- **Integration**: Produces Hamiltonians compatible with `fusion_quantum_sdk`

## Usage

```rust
use fusion_jordan_wigner::transform;
use fusion_quantum_sdk::Hamiltonian;

// Define Fermionic Hamiltonian
let fermi_ham = Hamiltonian::new(); 
// ... add terms ...

// Transform to Qubit Hamiltonian
let qubit_ham = transform(&fermi_ham)?;
```

## Dependencies

- `fusion_core`
- `num_complex`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
