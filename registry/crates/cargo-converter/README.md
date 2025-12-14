# Cargo Converter

**Version:** 0.2.0  
**Type:** Interop Tool  
**License:** MIT

## Overview

Cargo Converter (`fusion_interop_cargo_converter`) is a tool that translates Rust `Cargo.toml` metadata and type definitions into Fusion-compatible FFI definitions. This allows seamless usage of Rust crates within Fusion projects.

## Features

- **Metadata Extraction**: Reads dependency trees from `Cargo.lock`
- **Type Mapping**: Auto-generates Fusion struct/enum definitions from Rust code
- **Build Integration**: Hooks into `fusion build` to automate binding generation

## Usage

```bash
# Generate Fusion bindings for a Rust crate
fusion cargo-convert --manifest-path ./rust-lib/Cargo.toml --output ./fusion-lib/
```

## Dependencies

- `fusion_core`
- `toml`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
