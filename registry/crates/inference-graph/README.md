# Fusion Inference Graph

**Version:** 0.2.0
**Type:** AI Orchestration
**License:** MIT

## Overview

Fusion Inference Graph (`fusion_inference_graph`) manages the execution flow of complex AI inference pipelines. It allows you to model, optimize, and execute Directed Acyclic Graphs (DAGs) of ML operations.

## Features

- **DAG Modeling**: Construct inference pipelines as graphs
- **Optimization**: Topological sorting and parallel execution planning
- **Integration**: Works with `fusion_ai_core` models and tensors
- **Visualization**: Export graphs to DOT format

## Usage

```rust
use fusion_inference_graph::{Graph, Node};

let mut graph = Graph::new();
let input = graph.add_node(Node::Input("image"));
let preprocess = graph.add_node(Node::Op("normalize"));
let model = graph.add_node(Node::Model("resnet50"));

graph.add_edge(input, preprocess);
graph.add_edge(preprocess, model);

let result = graph.execute(data).await?;
```text

## Dependencies

- `fusion_core`
- `fusion_ai_core`
- `petgraph`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)