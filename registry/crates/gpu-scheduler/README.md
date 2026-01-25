# Fusion GPU Scheduler

**Version:** 0.2.0
**Type:** Core Service
**License:** MIT

## Overview

Fusion GPU Scheduler (`fusion_llm_gpu_scheduler`) optimizes the allocation of GPU resources, specifically VRAM, for concurrent LLM serving and compute workloads. It prevents Out-Of-Memory (OOM) errors and maximizes GPU utilization.

## Features

- **VRAM Virtualization**: Manages virtual memory pages on GPU
- **Optimistic Allocation**: Allows over-provisioning with fast swapping
- **Priority Scheduling**: Prioritizes interactive tokens over background batch jobs
- **Multi-GPU**: Orchestrates memory across multiple GPUs

## Usage

```rust
use fusion_llm_gpu_scheduler::{Scheduler, AllocationRequest};

let scheduler = Scheduler::new();
let request = AllocationRequest::new("model-70b", 40 * 1024 * 1024 * 1024); // 40GB

if let Ok(handle) = scheduler.allocate(request).await {
    // Run GPU kernel
}
```text

## Dependencies

- `fusion_core`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)