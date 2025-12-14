# fusion-sandbox

Secure execution environment for untrusted Fusion code.

## Features

- Process isolation
- Resource limiting
- System call filtering

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-sandbox = "0.1.0"
```

## Usage

```rust
use fusion_sandbox::hello;

fn main() {
    hello();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
