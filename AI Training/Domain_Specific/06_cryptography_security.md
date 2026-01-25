# Cryptography and Security

**Dataset Category**: Domain Specific
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

Fusion enforces hybrid crypto by default with PQC algorithms and constant‑time primitives.

## Example

```fusion
@constant_time
fn secure_compare(a: &[u8], b: &[u8]) -> bool { /* ... */ }
```text