# Fusion JS Interop

**Version:** 0.2.0
**Type:** Language Bridge
**License:** MIT

## Overview

Fusion JS Interop (`interop-js`) enables bi-directional interoperability between Fusion and JavaScript. It supports embedding V8 (via Deno core) or QuickJS to run JS code within Fusion applications.

## Features

- **Runtime Embedding**: Execute JS scripts inside Fusion
- **Value Conversion**: Serde-based conversion between generic Values
- **Module Support**: Load CommonJS or ES Modules
- **Host Functions**: Expose Fusion functions to JavaScript

## Usage

```rust
use interop_js::{Runtime, Value};

let mut runtime = Runtime::new();
runtime.execute("const add = (a, b) => a + b;")?;

let result: i32 = runtime.call("add", &[10, 20])?;
assert_eq!(result, 30);
```text

## Dependencies

- `fusion_core`
- `deno_core` (optional)
- `rquickjs` (optional)

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)