# Fusion Compiler Passes

**Version:** 0.2.0  
**Type:** Compiler Plugin  
**License:** MIT

## Overview

Fusion Compiler Passes (`fusion_q_compiler_pass`) implements advanced transformation passes for the Fusion compiler, specifically focused on Quantum circuit optimization and high-level classical code lowering.

## Features

- **Gate Equivalence**: Replaces gate sequences with unexpected identities
- **Basis Transformation**: Converts abstract gates to hardware-native gate sets
- **Dead Code Elimination**: Removes unused quantum registers and classical logic
- **Constant Folding**: Evaluates constant expressions at compile time

## Usage

Used internally by the Fusion compiler (`fusion_core`):

```rust
use fusion_q_compiler_pass::{PassManager, GateOptimizationPass};

let mut pm = PassManager::new();
pm.add_pass(GateOptimizationPass::default());
pm.run(&mut ir_module)?;
```

## Dependencies

- `fusion_core`
- `fusion_quantum_sdk`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
