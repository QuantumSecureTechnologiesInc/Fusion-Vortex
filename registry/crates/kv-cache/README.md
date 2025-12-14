# Fusion KV Cache

**Version:** 0.2.0  
**Type:** Data Storage  
**License:** MIT

## Overview

Fusion KV Cache (`fusion_kv_cache`) is a high-performance in-memory Key-Value store designed for caching intermediate results, LLM attention states (KV-Cache), and session data.

## Features

- **Zero-Copy**: Optimized for tensor storage without copying
- **Eviction**: LRU, LFU, and TTL-based eviction policies
- **Concurrency**: Lock-free reads for high throughput
- **Persistence**: Optional snapshotting to disk

## Usage

```rust
use fusion_kv_cache::Cache;

let cache = Cache::new(1024 * 1024 * 1024); // 1GB
cache.insert("key", tensor).await;

if let Some(val) = cache.get("key").await {
    // use val
}
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
