# Fusion Std

**Version:** 0.2.0  
**Type:** Core Library  
**License:** MIT

## Overview

Fusion Std (`fusion_std`) is the standard library extension for Fusion. It provides essential utilities, error handling primitives, and common types that extend the Rust standard library for the Fusion ecosystem.

## Features

- **Error Handling**: Unified `FusionError` and `Result` types
- **Async I/O**: Extensions for async read/write operations
- **Serialization**: Common Serde helpers
- **System**: Platform abstraction layer

## Usage

```rust
use fusion_std::error::FusionResult;
use fusion_std::fs::AtomicFile;

fn save_config() -> FusionResult<()> {
    let file = AtomicFile::create("config.toml")?;
    file.write_all(b"key=value")?;
    Ok(())
}
```

## Dependencies

- `fusion_core`
- `thiserror`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
