# Fusion GraphQL

**Version:** 0.2.0
**Type:** API Server
**License:** MIT

## Overview

Fusion GraphQL (`fusion_api_graphql`) provides a high-performance, async GraphQL server implementation for Fusion applications. It integrates with `fusion_http` and `fusion_runtime_core`.

## Features

- **Schema First & Code First**: Supports both schema definition styles
- **Subscriptions**: Real-time updates via WebSockets
- **Federation**: Support for Apollo Federation 2.0
- **Dataloader**: N+1 query problem solver

## Usage

```rust
use fusion_api_graphql::{Schema, Object};

struct Query;

#[Object]

impl Query {
    async fn hello(&self) -> &str {
        "Hello Fusion"
    }
}

let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
```text

## Dependencies

- `async-graphql`
- `fusion_http`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)