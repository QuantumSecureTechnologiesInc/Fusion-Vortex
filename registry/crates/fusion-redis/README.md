# Fusion Redis

A Redis-compatible server implementation built on top of the Fusion Runtime Core.

## Features

- **High Performance**: Uses `fusion_runtime_core` for task scheduling and `fusion_net` for networking.
- **RESP Protocol**: Full support for the Redis Serialization Protocol (RESP).
- **Core Commands**: Supports `SET`, `GET`, `DEL`, `EXISTS`, `PING`, `ECHO`.
- **In-Memory Store**: Uses concurrent hash maps for fast data access.

## Architecture

- `server.rs`: Handles TCP connections and task spawning via `fusion_runtime_core`.
- `resp.rs`: RESP protocol parser and serializer.
- `command.rs`: Command dispatch logic.
- `store.rs`: Thread-safe in-memory key-value store with expiration support.

## Usage

To run the server, instantiate `RedisServer` with a `Runtime` instance:

```rust
use fusion_runtime_core::Runtime;
use fusion_redis::RedisServer;
use std::sync::Arc;

async fn main() -> Result<(), anyhow::Error> {
    let runtime = Arc::new(Runtime::new());
    let server = RedisServer::new("127.0.0.1:6379".to_string(), runtime)?;
    server.run().await?;
    Ok(())
}
```text