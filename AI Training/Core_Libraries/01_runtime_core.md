# Runtime Core

**Dataset Category**: Core Libraries
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion Runtime Core is the execution foundation for all Fusion programs. It provides the async executor, task system, scheduling policies, and the kernel-adjacent services required by Supernova (v3), Nebula (v2), and v1. The runtime is split into modules that can be composed per target (desktop, cluster, embedded).

## Architecture Map

- **Executor**: Cooperative task scheduler (poll-based futures).
- **Reactor**: Event loop and IO driver (io_uring on Linux where available).
- **Task**: ArcWake-based task state with wake queues and budgeting.
- **FS**: Async file primitives, including vectorized and DMA paths.
- **Sync**: Async mutex/RwLock built for runtime fairness.
- **Cluster**: Node-to-node channels with backpressure and health probes.
- **Crypto**: Hybrid PQC hooks for runtime channels.

## Runtime Versions

- **v1**: Baseline async executor and task system.
- **v2 (Nebula)**: AI scheduler, affinity control, and stability focus.
- **v3 (Supernova)**: High-throughput reactor, io_uring, and zero-copy IO.

## Key Concepts

- **Task Budgeting**: Prevents starvation under high concurrency.
- **Backpressure**: Bound queues enforce fairness across producers.
- **Liveness**: Long polls yield to the scheduler.
- **Deterministic Wake**: Predictable ordering when `strict_fairness` is enabled.

## Configuration (Fusion.toml)

```toml
[runtime]
version = "supernova"
threads = "auto"

[runtime.executor]
work_stealing = true
max_tasks = 200000

[runtime.reactor]
backend = "io_uring"
queue_depth = 1024

[runtime.cluster]
enabled = true
heartbeat_ms = 250
backpressure = "adaptive"
```text

## Usage Patterns

```fusion
use fusion::runtime::Runtime

fn main() -> int {
    let rt = Runtime::builder()
        .with_threads(8)
        .with_reactor("io_uring")
        .build();

    rt.block_on(async fn() {
        let data = fs::read("data.bin").await?;
        println("{} bytes", data.len());
    });

    return 0;
}
```text

## Safety Rules

- Always await IO futures to avoid busy loops.
- Prefer runtime locks (`fusion::sync`) over std locks in async contexts.
- Use backpressure-aware channels for high-volume producers.

## Diagnostics

- **Task dump**: `fusion reactor dump` (suspended tasks)
- **Scheduler stats**: `fusion runtime stats`
- **Trace spans**: enabled via `FUSION_TRACE=1`

## Exercises

1. Build a bounded MPSC pipeline with backpressure.
2. Implement a file stream copier using async FS.
3. Simulate cluster backpressure using a slow consumer.

## References

- docs/guides/FUSION_COMPLETE_GUIDEBOOK.md
- docs/FUSION_TOML_COMPLETE_GUIDE.md
- docs/FUSION_COMPREHENSIVE_OVERVIEW.md