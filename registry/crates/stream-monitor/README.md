# Fusion Stream Monitor

**Version:** 0.2.0
**Type:** Observability
**License:** MIT

## Overview

Fusion Stream Monitor (`fusion_llm_stream_monitor`) is a specialized observability tool for tracking the performance of streaming LLM responses. It measures time-to-first-token (TTFT), inter-token latency (ITL), and total throughput.

## Features

- **Real-time Monitoring**: Zero-overhead tracking of active streams
- **Metrics**: Automatic calculation of P50, P90, P99 latencies
- **Alerting**: Hooks for slow generation detection

## Usage

```rust
use fusion_llm_stream_monitor::StreamMonitor;

let mut monitor = StreamMonitor::new();
monitor.on_token_generated(); // Call on each token
monitor.finish();

println!("Throughput: {} t/s", monitor.throughput());
```text

## Dependencies

- `fusion_std`
- `chrono`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)