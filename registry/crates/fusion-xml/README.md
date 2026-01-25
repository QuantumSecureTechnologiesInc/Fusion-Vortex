# Fusion XML

**Version:** 0.2.0
**Type:** Data Format
**License:** MIT

## Overview

Fusion XML (`fusion-xml`) provides high-performance, safe XML parsing and serialization for the Fusion ecosystem. It is designed to be tolerant of malformed input and efficient for large documents.

## Features

- **DOM & SAI Parsing**: Supports both tree-based (DOM) and event-based (SAX-like) parsing
- **Type-Safe**: Maps XML to Fusion structs using serde-like macros
- **Validation**: Optional DTD and Schema validation
- **Streaming**: Efficient zero-copy streaming parser

## Usage

```rust
use fusion_xml::from_str;
use serde::Deserialize;

#[derive(Deserialize)]

struct Config {
    port: u16,
    host: String,
}

let xml = "<Config><port>8080</port><host>localhost</host></Config>";
let config: Config = from_str(xml)?;
```text

## Dependencies

- `fusion_core`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)