# Fusion Graph

**Version:** 0.2.0
**Type:** Data Structure
**License:** MIT

## Overview

Fusion Graph (`fusion_graph`) provides optimized graph data structures and algorithms. It is used for dependency resolution, semantic analysis, and network topology modeling within Fusion.

## Features

- **Directed & Undirected**: Supports multiple graph types
- **Weighted Edges**: Generic edge weights
- **Traversal**: BFS, DFS, Dijkstra, A*
- **Serialization**: Efficient graph serialization

## Usage

```rust
use fusion_graph::{Graph, NodeIndex};

let mut deps = Graph::new();
let a = deps.add_node("lib_a");
let b = deps.add_node("lib_b");
deps.add_edge(a, b, "depends_on");

for neighbor in deps.neighbors(a) {
    println!("{:?}", neighbor);
}
```text

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)