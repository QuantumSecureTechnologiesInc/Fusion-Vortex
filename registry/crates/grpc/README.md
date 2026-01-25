# Fusion gRPC

**Version:** 0.2.0
**Type:** RPC Framework
**License:** MIT

## Overview

Fusion gRPC (`fusion_server_grpc`) is a high-performance RPC framework based on HTTP/2 and Protocol Buffers. It provides seamless integration with the Fusion runtime for microservice architecture.

## Features

- **Code Generation**: Compiles `.proto` files to Rust traits
- **Authentication**: Interceptors for token validation
- **Streaming**: Bi-directional streaming RPCs
- **Reflection**: Support for gRPC Server Reflection

## Usage

```rust
// Defined in build.rs via prost
use my_proto::greeter_server::{Greeter, GreeterServer};

struct MyGreeter;

#[tonic::async_trait]

impl Greeter for MyGreeter {
    async fn say_hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            message: format!("Hello {}", req.get_ref().name),
        }))
    }
}
```text

## Dependencies

- `tonic`
- `prost`
- `fusion_net`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)