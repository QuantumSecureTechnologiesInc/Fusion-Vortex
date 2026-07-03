> **Phase 0 audit (2026-06-24) found this doc overclaims reality.**
> Treat feature lists here as roadmap, not current state.
> See `docs-truth-audit/TRUTH_REPORT.md` for details.

# Native Quantum Integration

## Overview
Fusion is the world's first **quantum-native** systems language. Unlike other languages that require external SDKs or string-based circuit definitions, Fusion embeds quantum types and operations directly into its syntax and standard library.

## Core Features

### ⚛️ First-Class Quantum Types
- `Qubit`, `QuantumCircuit`, `QuantumRegister` are primitive types.
- Compile-time checking of quantum operations (e.g., preventing cloning of qubits).

### ☁️ Backend Agnostic
Write code once, run anywhere. Fusion supports:
- **Simulators**: High-performance state vector & density matrix simulations.
- **Physical Hardware**: Seamless integration with:
    - AWS Braket
    - IBM Quantum
    - Google Quantum AI

### 🧮 Built-in Algorithms
Standard library support for:
- **QAOA** (Quantum Approximate Optimization Algorithm)
- **VQE** (Variational Quantum Eigensolver)
- **Shor's Algorithm** primitives
- **Grover's Search**

## Code Example

```fusion
use fusion::quantum::*;

fn create_bell_pair() -> QuantumCircuit {
    let mut c = QuantumCircuit::new(2);
    // Native gate syntax
    c.h(0);      // Hadamard
    c.cx(0, 1);  // Controlled-NOT
    c
}

async fn run_experiment() {
    let circuit = create_bell_pair();
    // Transparently runs on configured backend (Sim or Hardware)
    let result = execute(circuit).await?;
    println!("State: {:?}", result.state_vector());
}
```
