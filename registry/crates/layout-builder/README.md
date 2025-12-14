# Fusion Layout Builder

**Version:** 0.2.0  
**Type:** Developer Tool  
**License:** MIT

## Overview

Fusion Layout Builder (`fusion_ui_layout_builder`) is a visual tool and library for constructing UI layouts. It is used by the Fusion IDE and low-code builders to generate UI component trees.

## Features

- **Drag-and-Drop Logic**: Algorithms for snapping and placement
- **Grid System**: Flexible flexbox/grid layout engine
- **Serialization**: Save layouts to JSON/FusionUI format
- **Code Gen**: Export layouts to Rust/Fusion code

## Usage

```rust
use fusion_ui_layout_builder::{Builder, Element};

let mut builder = Builder::new();
builder.add(Element::new("Button").at(10, 10));
let layout = builder.build();
```

## Dependencies

- `fusion_ui_component_lib`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
