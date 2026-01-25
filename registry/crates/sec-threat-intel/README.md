# sec-threat-intel

Integration with threat intelligence feeds.

## Features

- Feed aggregation
- IOC matching
- Real-time updates

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sec-threat-intel = "0.1.0"
```text

## Usage

```rust
use sec_threat_intel::Client;

fn main() {
    Client::new().fetch();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.