
# Developer Guide

## Architecture

The compiler follows a standard pipeline:

1. **Lexer** (`src/lexer.rs`): Converts source to `Token` stream.
2. **Parser** (`src/parser.rs`): Consumes tokens to produce `AST` (`src/ast.rs`).
3. **Semantic Analysis**: Builds symbol tables (`src/semantic.rs`).
4. **Type Checker** (`src/type_checker.rs`): Validates the AST against type rules.

## Contributing

Run tests with `cargo test` (not implemented yet).
Run the CLI with `cargo run`.
