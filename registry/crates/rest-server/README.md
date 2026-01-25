# REST Server

A high-performance, asynchronous REST API server framework for Fusion.

## Features

- Routing and middleware
- JSON serialization/deserialization
- OpenAPI integration

## Usage

```rust
use rest_server::{Server, Route};

Server::new()
    .route("/", handler)
    .serve("127.0.0.1:8080").await?;
```text