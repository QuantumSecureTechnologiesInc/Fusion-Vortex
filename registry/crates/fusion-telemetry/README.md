# fusion-telemetry

Unified telemetry collection and reporting for Fusion applications.

## Features

- Metrics aggregation
- Distributed tracing
- Log correlation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-telemetry = "0.1.0"
```text

## Usage

```rust
use fusion_telemetry::init;

fn main() {
    init();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.