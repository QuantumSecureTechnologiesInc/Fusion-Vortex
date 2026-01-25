# model-server-core

Foundation for building high-performance model serving applications.

## Features

- HTTP/gRPC scaffold
- Batching scheduler
- Model versioning

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
model-server-core = "0.1.0"
```text

## Usage

```rust
use model_server_core::Server;

fn main() {
    Server::new().start();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.