# Fusion HTTP

**Version:** 0.2.0  
**Type:** Web Framework  
**License:** MIT

## Overview

Fusion HTTP (`fusion_http`) is a fast, asynchronous HTTP/1.1 and HTTP/2 client/server library. It leverages `fusion_runtime_core` for non-blocking I/O.

## Features

- **Client & Server**: Unifying abstractions for both ends
- **Middleware**: Tower-based middleware stack
- **HTTP/2**: Multiplexed connections by default
- **Bodies**: Efficient streaming body handling

## Usage

```rust
use fusion_http::{Server, Request, Response, Body};

async fn handler(_req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::bind("127.0.0.1:3000".parse()?);
    server.serve(handler).await?;
    Ok(())
}
```

## Dependencies

- `hyper`
- `tower`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
