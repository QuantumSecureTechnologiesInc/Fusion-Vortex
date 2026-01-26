# Web & Networking

## Overview
Fusion provides an "Async-First" networking stack inspired by the best of Node.js and Go, but with the performance of Rust. It removes the need for external frameworks (like Express or Actix) for standard web development.

## Capabilities

### 🌐 High-Performance Server
- **Protocol Support**: HTTP/1.1, HTTP/2, HTTP/3 (QUIC) out of the box.
- **gRPC**: Native support with Protocol Buffers; no plugins required.
- **WebSocket**: Real-time bidirectional communication.

### 🕸️ WebAssembly (WASM)
- **Frontend Support**: Compile Fusion code directly to WASM for the browser.
- **DOM Integration**: Type-safe manipulation of the browser DOM.

### 🛡️ Security
- **Post-Quantum TLS**: All connections use PQ-safe cipher suites by default.
- **Service Mesh**: Native service discovery and mTLS for microservices.

## Example: Web API

```fusion
use fusion::web::*;

#[fusion::main]
async fn main() {
    let app = Router::new()
        // Simple route
        .route("/", get(|| async { "Hello World" }))
        // JSON parsing
        .route("/users", post(create_user))
        // Middleware
        .layer(Cors::permissive());

    // Start HTTP/3 server
    Server::bind("0.0.0.0:8080")
        .serve(app)
        .await?;
}
```
