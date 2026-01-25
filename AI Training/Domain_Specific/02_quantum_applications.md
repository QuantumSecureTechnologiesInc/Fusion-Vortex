# Quantum Applications

**Dataset Category**: Domain Specific
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

Fusion supports quantum algorithms with classical/quantum hybrid execution and cloud backends.

## Example

```fusion
let mut c = Circuit::new(3);
c.h(0); c.cx(0,1); c.cx(1,2);
let result = c.run_sim();
```text

## Backends

- IBM Quantum
- AWS Braket