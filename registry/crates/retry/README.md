# retry

Flexible retry mechanisms and policies for Fusion applications.

## Features

- Exponential backoff
- Jitter support
- Async integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
retry = "0.1.0"
```text

## Usage

```rust
use retry::retry;

fn main() {
    retry(|| {
        println!("Attempting...");
        Ok(())
    });
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.