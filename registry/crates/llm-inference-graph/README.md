# Fusion LLM Inference Graph

**Version:** 0.2.0  
**Type:** Visualization/Debug  
**License:** MIT

## Overview

Fusion LLM Inference Graph (`llm-inference-graph`) builds on `fusion_inference_graph` to provide specific visualizations and debugging tools for LLM computational graphs (e.g., attention heads, layer activations).

## Features

- **Attention Maps**: Visualize attention weights across heads
- **Activation Tracing**: Track tensor values through layers for debugging
- **Throughput Analysis**: Identify bottlenecks in the inference pipeline

## Usage

```rust
use llm_inference_graph::Tracer;

let tracer = Tracer::new();
tracer.attach(model);
// Run inference
tracer.export("trace.json")?;
```

## Dependencies

- `fusion_inference_graph`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
