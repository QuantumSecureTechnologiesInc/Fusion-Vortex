# Systems Programming

**Dataset Category**: Domain Specific
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

Fusion supports low‑level programming with FFI and manual memory control.

## Example

```fusion
@manual_memory
fn alloc_buf(n: int) -> *void { alloc(n) }
```text