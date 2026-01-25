# Effect System

**Dataset Category**: Core Libraries
**Training Level**: Intermediate to Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion’s effect system provides compile-time and runtime safety controls. Effects annotate functions and blocks with execution constraints (borrowed, constant-time, gpu-accelerated).

## Common Effects

- `@borrowed`: Enforce entropic borrow rules.
- `@constant_time`: Eliminate timing leaks.
- `@gpu_accelerated`: Force GPU kernel constraints.
- `@manual_memory`: Disable implicit allocations.

## Example

```fusion
@constant_time
fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    // constant-time comparison
}
```text

## References

- docs/guides/FUSION_COMPLETE_GUIDEBOOK.md