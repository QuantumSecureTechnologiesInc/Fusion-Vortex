# sdk-generator

Generator for Fusion client SDKs in various languages.

## Features

- Supports Rust, Go, TypeScript
- Auto-generated documentation
- REST and gRPC support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sdk-generator = "0.1.0"
```text

## Usage

```rust
use sdk_generator::Gen;

fn main() {
    Gen::new().run();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.