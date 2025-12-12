# Flux-Resolve Engine

**Location:** `runtime/crates/fusion_flux_resolve`  
**Status:** ✅ Migrated to Fusion Runtime  
**Version:** 0.3.0

## Overview

The Flux-Resolve Engine is a Fusion-native dependency resolution module that has been properly relocated to the Fusion runtime workspace. It provides:

- VSIDS (Variable State Independent Decaying Sum) heuristics
- DFS-based cycle detection
- Content-addressable storage (L1/L2 cache)
- Adaptive GPU offloading
- Performance telemetry

## Architecture

This module follows the Fusion architecture pattern:

```
Fusion Module (stdlib/flux_resolve.fu) - Core logic in Fusion
    ↓ FFI
Rust Bridge (runtime/crates/fusion_flux_resolve) - System operations
    ↓
OS (File I/O, GPU, Network)
```

## Components

### Rust Bridge (`fusion_flux_resolve`)

Provides FFI exports for:
- **CacheBridge** - File I/O for L2 disk cache
- **GpuBridge** - CUDA kernel loading
- **RegistryBridge** - HTTP requests to package registry

### FFI Exports

```rust
extern "C" fn flux_resolve_bridge_create() -> *mut FluxResolveBridge
extern "C" fn flux_resolve_bridge_destroy(bridge: *mut FluxResolveBridge)
extern "C" fn flux_resolve_cache_get(...) -> *mut u8
extern "C" fn flux_resolve_cache_put(...)
```

## Building

```bash
cd runtime
cargo build -p fusion_flux_resolve
cargo test -p fusion_flux_resolve
```

## Integration

This crate is part of the Fusion runtime workspace and will be linked with the Fusion compiler/runtime to provide dependency resolution capabilities.

## Configuration

Environment variables:
- `FUSION_CUDA_ENABLE` - Enable GPU acceleration (default: true)
- `FUSION_REGISTRY_URL` - Package registry URL

Default config:
- GPU threshold: 10,000 nodes
- VSIDS decay: 0.95
- Cache path: `.fusion/cache_db`

## See Also

- Fusion Runtime documentation
- `fusion_runtime_core` - Core runtime
- `fusion_traits` - Shared traits
