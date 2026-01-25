# math-finite-fields

FFT-friendly finite field arithmetic for cryptography and zero-knowledge proofs.

## Features

- Prime fields
- Extension fields
- Efficient arithmetic

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
math-finite-fields = "0.1.0"
```text

## Usage

```rust
use math_finite_fields::FieldElement;

fn main() {
    let a = FieldElement::new(5);
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.