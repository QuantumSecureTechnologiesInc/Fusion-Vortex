# Fusion LLM RAG

**Version:** 0.2.0  
**Type:** Application Framework  
**License:** MIT

## Overview

Fusion LLM RAG (`fusion_llm_rag`) is a lightweight, thread-safe library for building Retrieval-Augmented Generation systems. It provides a local vector store and retrieval pipeline.

## Features

- **Store**: In-memory or on-disk vector storage (HNSW)
- **Retrieval**: Semantic similarity search with metadata filtering
- **Chunking**: Document splitting utilities
- **Concurrency**: Safe for concurrent reads/writes

## Usage

```rust
use fusion_llm_rag::{VectorStore, Document};

let mut store = VectorStore::new();
store.add(Document::new("Fusion is fast.", vec![0.1, 0.2]))?;

let results = store.search(query_vec, 5)?;
```

## Dependencies

- `fusion_core`
- `thiserror`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
