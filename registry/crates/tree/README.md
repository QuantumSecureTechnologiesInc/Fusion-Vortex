# Fusion Tree

**Version:** 0.2.0  
**Type:** Data Structures  
**License:** MIT

## Overview

Fusion Tree (`fusion_tree`) provides high-performance tree data structures used throughout the Fusion compiler and runtime. It includes B-Trees, Red-Black Trees, and specialized AST implementations.

## Features

- **B-Tree**: Cache-friendly search tree
- **AST Node**: Generic nodes for syntax trees
- **Traversals**: Efficient iterator-based traversals

## Usage

```rust
use fusion_tree::BTree;

let mut tree = BTree::new();
tree.insert(1, "one");
tree.insert(2, "two");

assert_eq!(tree.get(&1), Some(&"one"));
```

## Dependencies

- `fusion_core`
- `fusion_std`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
