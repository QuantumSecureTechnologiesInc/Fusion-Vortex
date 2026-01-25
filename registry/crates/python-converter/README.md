# Python Converter

A utility crate for converting Python code and data structures into Fusion equivalents.

## Features

- AST bridging between Python and Fusion
- Type conversion utilities
- Interop helpers

## Usage

```rust
use python_converter::Converter;

let fusion_ast = Converter::convert_source(python_code)?;
```text