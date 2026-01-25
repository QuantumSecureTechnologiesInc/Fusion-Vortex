# Interop and FFI

**Dataset Category**: Core Libraries
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0) (v1.0.0)

---

## Overview

Fusion supports FFI through `extern fn` declarations and ABI policies. By-pointer ABI rewriting is used for aggregates where needed.

## Example

```fusion
extern fn printf(fmt: string, val: int) -> int;

fn main() -> int {
    printf("value=%d
", 42);
    return 0;
}
```text

## ABI Notes

- Aggregates passed by pointer when required by ABI policy.
- String values map to `i8*` in LLVM (UTF-8).

## References

- docs/FUSION_TOML_COMPLETE_GUIDE.md
- docs/FUSION_COMPREHENSIVE_OVERVIEW.md