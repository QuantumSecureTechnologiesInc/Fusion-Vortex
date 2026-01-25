# Fusion AI Daemon

**Version:** Workspace
**Type:** Background Service
**License:** MIT / Apache 2.0 Dual License

## Overview

The Fusion AI Daemon (`fusion-ai-daemon`) is a persistent background service responsible for managing heavy AI workloads. It handles model loading, inference queueing, caching, and resource allocation for local LLMs and embeddings.

## Purpose

- **Resource Management**: Keeps models loaded in VRAM to avoid load latency
- **Queue Management**: Serializes requests to prevent OOM errors
- **Caching**: Caches common embeddings and completions
- **Offloading**: Manages CPU/GPU/NPU offloading automatically

## Features

- **Persistent State**: Maintains model state across CLI commands
- **HTTP API**: Exposes internal API for tools to request inference
- **Automatic Shutdown**: Scales down after inactivity to free resources
- **Multi-Model Support**: Can manage multiple loaded models simultaneously

## Usage

The daemon is typically managed automatically by `fusion-ai-cli` or other tools, but can be run manually:

```bash
fusion-ai-daemon --port 1234 --model-dir /path/to/models
```text

## API

Internal endpoints:
- `POST /v1/chat/completions` - OpenAI-compatible chat API
- `POST /v1/embeddings` - Vector generation
- `GET /v1/status` - Daemon health and resource usage

## Integration

Designed to work with `fusion-ai-models` for local inference execution.

## Dependencies

- `fusion-ai-core`
- `fusion-ai-models`
- `hyper` / `tower` (HTTP stack)

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)