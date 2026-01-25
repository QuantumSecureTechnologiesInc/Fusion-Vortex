# Fusion Telemetry Ingestor

**Version:** 0.2.0
**Type:** Observability
**License:** MIT

## Overview

Fusion Telemetry Ingestor (`fusion_telemetry_ingestor`) is a centralized service for collecting, aggregating, and forwarding metrics and logs from distributed Fusion nodes.

## Features

- **Prometheus Exporter**: Exposes scraped metrics in Prometheus format
- **Log Aggregation**: Collects structured logs from services
- **Tracing**: Ingests distributed trace spans
- **Sampling**: Configurable sampling rates for high-volume telemetry

## Usage

```rust
use fusion_telemetry_ingestor::{Ingestor, Config};

let config = Config::default();
let ingestor = Ingestor::new(config);
ingestor.start_server("0.0.0.0:9090").await?;
```text

## Dependencies

- `fusion_core`
- `fusion_net`
- `prometheus`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)