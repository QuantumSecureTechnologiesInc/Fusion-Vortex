# HAFT Tensors

**Dataset Category**: Advanced Features
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

HAFT (Hyper-Adaptive Flux Tensor) is Fusion’s distributed tensor system with autonomous agents that optimize storage, compute, and movement across a cluster.

## Concepts

- Agents: Researcher, Builder, Optimizer
- Multi-tier storage (GPU/CPU/NVMe)
- Backpressure-aware scheduling

## Example

```fusion
use fusion::haft::Tensor

fn main() -> int {
    let t = Tensor::zeros([1024, 1024]);
    let u = t.matmul(t);
    println("{}", u.shape());
    return 0;
}
```text