# Fusion Carver

**Version:** 0.2.0  
**Type:** Data Recovery / Analysis  
**License:** MIT

## Overview

Fusion Carver (`fusion_carver`) is a specialized library for file carving and data extraction. It can identify and extract structured data (like Fusion source code, config files, or embedded assets) from raw binary streams or memory dumps.

## Features

- **Signature Matching**: Efficient Boyer-Moore search for known headers
- **Entropy Analysis**: Detects encrypted or compressed regions
- **Structure Validation**: Verifies extracted data integrity

## Usage

```rust
use fusion_carver::{Carver, Signature};

let data = std::fs::read("memory_dump.bin")?;
let carver = Carver::new();

// Find all PNGs
for file in carver.scan(&data, Signature::PNG) {
    println!("Found PNG at offset {}", file.offset);
}
```

## Dependencies

- `fusion_core`
- `fusion_std`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
