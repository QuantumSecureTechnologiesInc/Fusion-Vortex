# Sentinel TriBrid

**Dataset Category**: Advanced Features
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Sentinel TriBrid is Fusion’s hybrid security monitor that evaluates CPU, network, and thermal telemetry to detect anomalies and isolate nodes.

## Example

```fusion
fn scan_node(n: Node) -> void {
    let score = n.cpu.delta() + n.net.jitter() + n.temp.rise();
    if score > 0.85 { Network::isolate(n); }
}
```text