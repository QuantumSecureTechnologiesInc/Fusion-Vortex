# Fusion Client

**Version:** 0.2.0
**Type:** SDK Core
**License:** MIT

## Overview

Fusion Client (`fusion_client`) is the primary entry point for applications interacting with the Fusion Runtime. It provides a high-level API for connecting to local or remote Fusion nodes, submitting tasks, and receiving results.

## Features

- **Connection Management**: Persistent connections with auto-reconnect
- **Task Submission**: Async API for submitting compute jobs (AI, Quantum, Classical)
- **Event Streaming**: Subscribe to runtime events and telemetry
- **Security**: TLS-encrypted communication

## Usage

```rust
use fusion_client::Client;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    let client = Client::connect("127.0.0.1:9000").await?;

    // Submit a job
    let job_id = client.submit_code("print('Hello Fusion')").await?;

    // Await result
    let result = client.wait_for_job(job_id).await?;
    println!("Output: {}", result.stdout);
    Ok(())
}
```text

## Dependencies

- `fusion_net`
- `fusion_core`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)