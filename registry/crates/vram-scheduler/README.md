# Fusion VRAM Scheduler

**Version:** 0.2.0
**Type:** System Service
**License:** MIT

## Overview

Fusion VRAM Scheduler (`fusion_vram_scheduler`) acts as the kernel-level arbitrator for GPU memory. While `gpu-scheduler` handles high-level job placement, this crate manages low-level paging and fragmentation.

## Features

- **Defragmentation**: Compacting memory blocks
- **Paging**: Unified Virtual Memory support
- **Monitoring**: Real-time hygiene statistics

## Usage

```rust
use fusion_vram_scheduler::{VramManager, Device};

let manager = VramManager::new(Device::Cuda(0));
let ptr = manager.alloc(1024)?;
```text

## Dependencies

- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)