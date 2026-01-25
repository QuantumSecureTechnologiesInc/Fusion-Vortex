# Fusion LLM Mixtral Routing

**Version:** 0.2.0
**Type:** Inference Logic
**License:** MIT

## Overview

Fusion LLM Mixtral Routing (`fusion_llm_mixtral_routing`) implements the router logic for Sparse Mixture of Experts (SMoE) models like Mixtral 8x7B. It handles the efficient dispatch of tokens to experts.

## Features

- **Top-2 Gating**: Efficient implementation of Top-K routing
- **Load Balancing**: Auxiliary loss calculation for expert balancing
- **Sparse Matmul**: Optimized kernels for sparse execution

## Usage

```rust
use fusion_llm_mixtral_routing::Router;

let router = Router::new(num_experts, hidden_dim);
let (expert_weights, expert_indices) = router.route(hidden_states)?;
```text

## Dependencies

- `fusion_core`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)