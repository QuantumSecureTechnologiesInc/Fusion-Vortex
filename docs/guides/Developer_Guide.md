# Fusion Developer Guide

## Architecture Overview

Fusion uses a classic compiler architecture with modern enhancements:

1. **Frontend**: ANTLR4-based lexer/parser generating an AST.
2. **Middle-end**: Semantic analysis, type checking, and optional borrow checking.
3. **Backend**: LLVM IR generation and optimization.

## Building from Source

### Prerequisites

- Rust (latest stable)
- LLVM 16+
- Python 3 (for test scripts)

### Build Command

```bash
cargo build --release
```

## Contributing

1. Fork the repository.
2. Create a feature branch.
3. Ensure all tests pass: `cargo test`
4. Submit a Pull Request.

## Coding Standards

- Rust code follows strict clippy guidelines.
- Fusion code examples must be syntactically correct according to the latest spec.
