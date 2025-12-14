# Diagnostics

A comprehensive diagnostics library for Fusion, offering error reporting, traversing, and formatting capabilities.

## Features
- Rich error reporting
- Span highlighting
- Suggestions and quick fixes

## Usage
```rust
use diagnostics::Diagnostic;

let diag = Diagnostic::error("Something went wrong")
    .with_span(span)
    .with_note("Did you mean...?");
```
