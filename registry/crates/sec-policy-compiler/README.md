# sec-policy-compiler

Compiler for translating high-level security policies into enforceable rules.

## Features

- Policy language parsing
- Rule optimization
- Target-agnostic backend

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sec-policy-compiler = "0.1.0"
```

## Usage

```rust
use sec_policy_compiler::Compiler;

fn main() {
    Compiler::compile();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
