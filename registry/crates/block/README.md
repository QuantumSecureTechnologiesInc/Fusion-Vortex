# Fusion Block

**Version:** 0.2.0  
**Type:** Blockchain Core  
**License:** MIT

## Overview

Fusion Block (`fusion_block`) defines the fundamental data structures for Fusion's blockchain integration. It provides the `Block`, `Header`, and `Transaction` primitives used by `fusion-blockchain` and related crates.

## Features

- **Block Structure**: Standardized block definition
- **Serialization**: Efficient binary serialization for network transport
- **Validation**: Core validation logic for block integrity
- **Hashing**: Merkle tree and block hashing utilities

## Usage

```rust
use fusion_block::{Block, Transaction};

// Create a block
let block = Block::new(parent_hash, timestamp, transactions);

// Calculate hash
let hash = block.hash();
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
