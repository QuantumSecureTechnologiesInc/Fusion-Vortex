# Universal GPU Interface

The **Universal GPU Interface** abstracts away the details of hardware acceleration on CPUs, NVIDIA GPUs and AMD GPUs.  It provides a single API for allocating contexts, generating entropy or keys, and querying capabilities.  The design follows the principles laid out in the integration documents and aims to balance performance, portability and security.

## Contexts

All work is performed within a *context* (`hc_context_t`).  A context encapsulates resources such as GPU streams and preallocated device memory.  Before generating any data you must initialise a context using the backend’s `init_context` function:

```c
hc_context_config_t cfg = {
    .device_id = -1,        // select the best available device automatically
    .stream_priority = 0,   // default stream priority
    .memory_pool_size = 0,  // let the backend decide
    .enable_profiling = true,
    .numa_node = 0          // prefer the first NUMA node on multi‑socket machines
};
hc_context_t ctx;
hc_gpu_status_t st = backend->init_context(&ctx, &cfg);
```

The configuration structure includes reserved space for future parameters so the ABI can evolve without breaking compatibility.

### NUMA Awareness

The `numa_node` field allows a caller to hint at the preferred NUMA node for host allocations.  When running on multi‑socket systems this can reduce cross‑socket latency when combined with pinned memory.

## Capabilities

After initialising a context you can query its capabilities via `get_caps`:

```c
hc_backend_caps_t caps;
backend->get_caps(ctx, &caps);
if (caps.supports_dma) {
    // backend can allocate pinned memory and perform DMA
}
if (caps.supports_avx512_ifma) {
    // CPU backend supports AVX‑512 integer fused multiply–add for extra speed
}
```

The `max_batch_size` and `optimal_batch_size` fields provide hints for how many items you should request in a single call to achieve peak throughput.

## Pinned Memory & DMA

GPU backends can allocate host‑pinned memory via `alloc_pinned`.  Pinned memory resides in RAM but is locked by the driver so it is never paged out.  When you pass a pinned pointer to `generate_batch` the backend performs an asynchronous DMA transfer from host to device, which can double throughput.  The backends also detect when user‑supplied buffers are already pinned and automatically choose the optimal copy strategy.

```c
uint8_t *buffer = backend->alloc_pinned(ctx, count * 32);
backend->generate_batch(ctx, seeds, blind, buffer, count, HC_FLAG_ENABLE_BLINDING | HC_FLAG_NON_BLOCKING);
// ... you may perform other work here ...
backend->sync(ctx); // wait for asynchronous generation to complete
backend->free_pinned(ctx, buffer);
```

## Generation Flags

The `generate_batch` function accepts a bitmask of flags that tailor its behaviour:

| Flag | Effect |
| --- | --- |
| `HC_FLAG_ENABLE_BLINDING` | Enables vacuum blinding by interleaving a second chaos trajectory.  This protects against timing and power side‑channel analysis at the cost of additional computation. |
| `HC_FLAG_NON_BLOCKING` | Launches the kernel asynchronously and returns immediately.  You must call `sync` before using the output. |
| `HC_FLAG_OPT_COMBINED_ROTATIONS` | Pre‑computes quaternion products when constructing KEM secrets. |
| `HC_FLAG_OPT_SANDWICH_ROTATION` | Enables the quaternion sandwich rotation (q v q⁻¹) for safe 3D rotation without gimbal lock. |
| `HC_FLAG_OPT_SCALAR_SPONGE` | Uses a register‑based sponge implementation rather than global memory for improved scalar performance. |
| `HC_FLAG_OPT_BRANCHLESS_LOGIC` | Avoids conditional branches inside hot loops for better instruction level parallelism. |
| `HC_FLAG_OPT_AVX512_IFMA` | On CPUs that support AVX‑512IFMA, uses vectorised instructions for the chaos map step. |

You can pass `HC_FLAG_OPT_MAX_PERFORMANCE` to enable all applicable optimisations.

## Telemetry & Introspection

When profiling is enabled in the context configuration, backends record telemetry about each batch generation.  You can retrieve this information via `get_telemetry`:

```c
hc_telemetry_t t;
backend->get_telemetry(ctx, &t);
printf("%lu batches, %lu keys total\n", t.total_batches, t.total_keys_generated);
printf("last batch took %.3f s\n", t.last_batch_time_sec);
```

The universal loader also exposes the semantic version of the active backend via `hc_gpu_get_version()` and converts status codes into descriptive strings via `hc_gpu_error_string()`.

## Dynamic Loading

The library uses `dlopen()`/`dlsym()` (on Linux) to load GPU plugins at runtime.  The CPU backend is always available as a fallback.  To use GPU acceleration you must place `libhc_cuda.so` or `libhc_rocm.so` in the same directory as your application.  The loader attempts to initialise the most capable backend first and falls back gracefully if initialisation fails.

## Thread Safety

Contexts are not thread‑safe.  You should either use separate contexts per thread or protect calls with a mutex.  The library itself does not perform any global locking beyond the initial backend selection.


