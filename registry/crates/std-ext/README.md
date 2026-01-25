# Fusion Std Ext

**Version:** 0.2.0
**Type:** Library Extension
**License:** MIT

## Overview

Fusion Std Ext (`fusion_std_ext`) provides additional specialized extensions to the standard library, focusing on numerical computing, complex numbers, and advanced data structures not present in the core `fusion_std`.

## Features

- **Complex Numbers**: Hardware-accelerated complex types
- **Math Extensions**: Special functions (erf, gamma, etc.)
- **Collections**: specialized collections (RingBuffer, bitsets)

## Usage

```rust
use fusion_std_ext::num::Complex;

let c1 = Complex::new(1.0, 2.0);
let c2 = Complex::new(3.0, 4.0);
let result = c1 * c2;
```text

## Dependencies

- `fusion_core`
- `num-complex`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)