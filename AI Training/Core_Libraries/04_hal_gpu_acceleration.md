# HAL GPU Acceleration

**Dataset Category**: Core Libraries
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

HAL abstracts GPU devices for tensor workloads and kernel launches. It supports CUDA (NVIDIA) and ROCm (AMD), plus CPU fallbacks.

## Key APIs

- `device::enumerate()`
- `tensor::allocate(shape, dtype)`
- `kernel::launch(name, grid, block)`

## Example

```fusion
use fusion::hal::gpu

fn main() -> int {
    let dev = gpu::default_device()?;
    let t = gpu::tensor::zeros([1024, 1024], "f32");
    gpu::kernel::launch("matmul", t);
    return 0;
}
```text

## Configuration

```toml
[gpu]
backend = "cuda"
streaming = true
```text

## References

- docs/guides/FUSION_COMPLETE_GUIDEBOOK.md