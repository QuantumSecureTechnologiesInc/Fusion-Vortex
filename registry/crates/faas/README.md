# Fusion FaaS

Function-as-a-Service (FaaS) utilities for Fusion.

## Features

- Function handlers
- Request/Response wrapping
- Cold start optimization helpers

## Usage

```rust
use faas::handler;

#[handler]

async fn my_func(req: Request) -> Response { ... }
```text