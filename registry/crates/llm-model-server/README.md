# Fusion LLM Model Server

**Version:** 0.2.0
**Type:** Application Server
**License:** MIT

## Overview

Fusion LLM Model Server (`fusion_llm_model_server`) is a production-grade HTTP/gRPC server for deploying Fusion LLMs. It bundles the Inference Engine, Batch Scheduler, and LoRA Manager into a deployable microservice.

## Features

- **OpenAI Compatible**: Drop-in replacement for OpenAI API
- **Metrics**: Prometheus metrics for token speed, latency, queue depth
- **Resiliency**: Graceful degradation and health checking
- **Scaling**: K8s-ready with readiness/liveness probes

## Usage

Run as a binary:

```bash
fusion-model-server --model llama-3-8b --port 8080
```text

Or embed in code:

```rust
use fusion_llm_model_server::Server;

let server = Server::builder()
    .model("llama-3-8b")
    .build()?;
server.run().await?;
```text

## Dependencies

- `fusion_http`
- `axum`
- `fusion_llm_inference_engine`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)