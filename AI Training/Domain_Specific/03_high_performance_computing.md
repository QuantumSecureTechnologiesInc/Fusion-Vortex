# High-Performance Computing

**Dataset Category**: Domain Specific
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

Fusion targets HPC with GPU kernels, SIMD, and distributed tensor execution.

## Example

```fusion
@gpu_kernel
fn vector_add(a: &[f32], b: &[f32]) -> Tensor<f32> { /* ... */ }
```text