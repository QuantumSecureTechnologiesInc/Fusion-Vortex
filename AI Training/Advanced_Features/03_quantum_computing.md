# Quantum Computing

**Dataset Category**: Advanced Features
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion provides native quantum circuit types, simulators, and hardware backends. Quantum operations are hybrid-secure and compatible with IBM Quantum and AWS Braket.

## Example

```fusion
use fusion::quantum::Circuit

fn main() -> int {
    let mut c = Circuit::new(2);
    c.h(0);
    c.cx(0, 1);
    let result = c.run_sim();
    println(result);
    return 0;
}
```text

## Hardware Backends

- IBM Quantum (Qiskit Runtime)
- AWS Braket (Rigetti, IonQ, OQC)