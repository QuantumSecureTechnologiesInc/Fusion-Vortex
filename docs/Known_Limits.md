# Fusion Known Limits (v1.0)

This document lists **known constraints** that are acceptable for v1.0 production.

## Language / Compiler

- Some advanced Rust‑style attributes are not parsed by the Fusion frontend.
- `use`/`mod` blocks are skipped for compatibility but should be avoided in core `.fu` sources.
- Parser error recovery is present but not exhaustive for malformed nested structures.

## Runtime

- `panic()` is implemented via a C runtime abort; no unwind or recovery.
- Deterministic failure messages are guaranteed for bounds checks.

## Interop

- Interop assets are stored under `toolchain/interop` and **not** compiled unless wrapped.
- ABI bridges are required for all non‑Fusion code.

## Toolchain

- LTO in mixed‑language builds requires explicit enablement and toolchain alignment.
- Some warnings may appear from optional backends (LLM/Q backends) but do not affect correctness.
