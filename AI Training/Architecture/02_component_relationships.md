# Component Relationships

**Dataset Category**: Architecture
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

This module maps the dependency graph of Fusion’s runtime, stdlib, HAFT, and tooling layers.

## Key Relationships

- fusion_std depends on fusion_runtime_core
- fusion_runtime_core depends on fusion_runtime_hal
- HAFT nodes depend on flux/resolve + tensorweave
- fusion CLI depends on sysroot + runtime artifacts

## Diagram (Text)

```text
stdlib -> runtime_core -> runtime_hal -> qpu backends
haft nodes -> flux -> tensorweave -> runtime_core
```text

## References

- docs/DocumentIndex.md