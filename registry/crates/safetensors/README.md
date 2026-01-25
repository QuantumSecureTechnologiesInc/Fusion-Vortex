# safetensors

Safe and fast tensor serialization for Fusion AI models.

## Features

- Zero-copy deserialization
- Memory safety guarantees
- Compatible with major frameworks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
safetensors = "0.1.0"
```text

## Usage

```rust
use safetensors::load;

fn main() {
    load("model.safetensors");
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.