# Async Runtime

**Dataset Category**: Core Libraries
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

The async runtime provides the Future/Task model, cooperative scheduling, and async IO primitives. It is the foundation for Supernova and Nebula.

## Concepts

- **Future**: Lazy computation polled by the executor.
- **Task**: A future plus scheduling metadata.
- **Waker**: Notifies executor that a task can run.
- **Reactor**: IO driver that wakes tasks on readiness.

## Example

```fusion
async fn fetch_data(path: string) -> bytes {
    let data = fs::read(path).await?;
    return data;
}

fn main() -> int {
    let rt = Runtime::default();
    let data = rt.block_on(fetch_data("input.bin"));
    println("{} bytes", data.len());
    return 0;
}
```text

## Patterns

- **Structured concurrency**: group tasks and await all.
- **Timeouts**: wrap futures with timeout helpers.
- **Streams**: async iterators for IO/data pipelines.

## Safety

- Avoid blocking inside async tasks.
- Use runtime locks, not OS mutexes.

## References

- docs/guides/FUSION_COMPLETE_GUIDEBOOK.md