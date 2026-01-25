# Fusion Trie Search

**Version:** 0.2.0
**Type:** Algorithms
**License:** MIT

## Overview

Fusion Trie Search (`fusion_llm_trie_search`) implements efficient Trie data structures for constrained decoding in LLMs. It enables features like schema enforcement and guided generation by restricting the next valid tokens.

## Features

- **Constrained Decoding**: Only allow tokens that traverse strict paths
- **Prefix Matching**: Fast lookup for autocomplete
- **Compressed Trie**: Memory-efficient storage of vocabularies

## Usage

```rust
use fusion_llm_trie_search::Trie;

let mut trie = Trie::new();
trie.insert(&[1, 2, 3]);
trie.insert(&[1, 2, 4]);

let valid_next = trie.get_next_options(&[1, 2]);
assert!(valid_next.contains(&3) && valid_next.contains(&4));
```text

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)