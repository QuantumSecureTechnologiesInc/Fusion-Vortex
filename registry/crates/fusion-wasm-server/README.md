# fusion-wasm-server

Server implementation for hosting Fusion WASM modules.

## Features

- Multi-tenant hosting
- Low-latency invocation
- Automatic scaling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-wasm-server = "0.1.0"
```

## Usage

```rust
use fusion_wasm_server::Server;

fn main() {
    Server::new().run();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
