# Wasm Server

A specialized web server for hosting and serving WebAssembly modules in the Fusion ecosystem.

## Features

- Efficient Wasm module serving
- Content-Type handling for `.wasm`
- Integration with Fusion's runtime

## Usage

```rust
use wasm_server::WasmServer;

WasmServer::new().serve_dir("./static").await?;
```text