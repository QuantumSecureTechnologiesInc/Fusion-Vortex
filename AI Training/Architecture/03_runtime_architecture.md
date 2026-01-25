# Runtime Architecture

**Dataset Category**: Architecture
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

Runtime architecture spans v1, Nebula v2, and Supernova v3. The runtime selects the backend via Fusion.toml and optional features.

## Layers

- Reactor (IO event loop)
- Executor (task scheduling)
- Sync primitives
- Cluster channels

## Configuration

```toml
[runtime]
version = "supernova"
threads = "auto"
```text