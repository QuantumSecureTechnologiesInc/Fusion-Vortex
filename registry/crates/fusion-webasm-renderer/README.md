# Fusion WebASM Renderer

**Version:** 0.2.0
**Type:** Web Tools
**License:** MIT

## Overview

Fusion WebASM Renderer (`fusion-webasm-renderer`) is a utility library for rendering detailed visual representations of WebAssembly modules. It allows developers to visualize WASM bytecode structure, memory layout, and control flow graphs.

## Features

- **Bytecode Visualization**: Visual representation of WASM instructions
- **Memory Mapping**: Displays linear memory usage and segment layout
- **Control Flow**: Generates CFGs for WASM functions
- **Interactive Output**: Output can be rendered to HTML/SVG

## Usage

```rust
use fusion_webasm_renderer::Renderer;

let wasm_bytes = std::fs::read("module.wasm")?;
let renderer = Renderer::new();

// Generate SVG visualization
let svg = renderer.render_svg(&wasm_bytes)?;
std::fs::write("module.svg", svg)?;
```text

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)