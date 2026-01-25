# sec-trusted-anchor

Hardware root-of-trust interface and trusted anchor management.

## Features

- TPM integration
- Secure enclave communication
- Root key management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sec-trusted-anchor = "0.1.0"
```text

## Usage

```rust
use sec_trusted_anchor::Anchor;

fn main() {
    Anchor::init();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.