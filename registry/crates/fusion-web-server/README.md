# fusion-web-server

Production-ready web server for Fusion web applications.

## Features

- HTTP/2 support
- Static file serving
- WebSocket integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-web-server = "0.1.0"
```text

## Usage

```rust
use fusion_web_server::serve;

fn main() {
    serve();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.