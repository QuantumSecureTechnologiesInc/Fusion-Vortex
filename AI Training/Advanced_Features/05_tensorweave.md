# TensorWeave

**Dataset Category**: Advanced Features
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

TensorWeave is Fusion’s distributed tensor fabric with erasure coding and adaptive routing for low-latency inference pipelines.

## Example

```fusion
fn recover(packets: Vec<TensorPacket>) -> Tensor {
    if packets.has_gaps() { Erasure::reconstruct(packets) } else { Tensor::from_packets(packets) }
}
```text