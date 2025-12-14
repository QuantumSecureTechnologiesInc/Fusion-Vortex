# Fusion Runtime Core v2.0 (Nebula)

**Version:** 2.0.0 "Nebula"  
**Status:** Active Development  
**Architecture:** Micro-kernel / WASM-based  
**Alias:** Can also be referenced as `nebula` in dependencies

## Overview

Fusion Runtime Core is a hyperscale execution engine designed for distributed systems. It allows for the safe, sandboxed execution of dynamic logic (Plugins) written in Rust and compiled to WebAssembly.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
# Using full name
fusion-runtime-core-v2-nebula = "2.0.0"

# Or using alias (requires workspace configuration)
nebula = { package = "fusion-runtime-core-v2-nebula", version = "2.0.0" }
```

Then import in your code:

```rust
// Using the alias
use nebula::{WasmEngine, fusion_proto};

// Or using full library name
use fusion_runtime_core_v2_nebula::{WasmEngine, fusion_proto};
```

## Project Structure

- **src/**: The Rust-based gRPC Host
  - `main.rs`: Entry point for the server binary
  - `lib.rs`: Library entry point
  - `engine.rs`: WASM execution logic (Wasmtime)
- **proto/**: Protocol Buffer definitions (.proto)
  - Strict contract for API interactions
- **sdk/**: Client libraries
  - `python/`: Python client for managing the core
- **examples/**: Reference implementations
  - `plugin/`: A basic Rust plugin template
- **docs/**: Documentation
  - `developer_guide.md`: How to build plugins

## Quick Start

### 1. Start the Server

```bash
cd registry/crates/fusion-runtime-core-v2-nebula
cargo run --bin fusion-runtime-server
```

### 2. Build the Example Plugin

```bash
cd examples/plugin
cargo build --target wasm32-unknown-unknown --release
```

### 3. Run the SDK Demo

```bash
cd sdk/python
# Ensure proto files are generated (see Developer Guide)
python fusion_client.py
```

## Features

- **Safety-First**: Fuel-based resource limits prevent infinite loops
- **Sandboxed Execution**: WASM provides memory isolation
- **gRPC API**: High-performance, language-agnostic communication
- **Hot-Swappable Plugins**: Update logic without restarting the core
- **Resource Monitoring**: Track execution time and resource usage
- **Convenient Aliasing**: Reference as `nebula` for shorter imports

## Dependencies

- **Rust 1.70+** with `wasm32-unknown-unknown` target
- **Protocol Buffers** compiler (for Python SDK)
- **Python 3.8+** with `grpcio` and `protobuf` (for SDK)

## Licence

MIT OR Apache-2.0
