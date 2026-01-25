# Fusion MoE Tools

**Version:** 0.2.0
**Type:** Diagnostics
**License:** MIT

## Overview

Fusion MoE Tools (`fusion_moe_diagnostics`) provides analysis and visualization tools for Mixture of Experts models. It helps debug expert utilization, routing collapse, and load imbalance.

## Features

- **Expert Heatmaps**: Visualize token routing distribution
- **Imbalance Metrics**: Calculate coefficient of variation for load
- **Dead Expert Detection**: Identify underutilized experts
- **Training Logs**: Parsers for MoE training logs

## Usage

```rust
use fusion_moe_diagnostics::Analyzer;

let analyzer = Analyzer::from_logs("training.log");
analyzer.report_imbalance();
```text

## Dependencies

- `fusion_core`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)