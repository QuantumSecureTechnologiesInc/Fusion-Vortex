# Fusion v2.0 Vortex Security Guarantees (Production Policy)

This document defines **what Fusion guarantees** and **what it does not**. It is part of the release gate.

## Guaranteed (when build passes all gates)

1. **Array bounds checks**
   - All array index operations are guarded at runtime.
   - Out‑of‑bounds access aborts via `panic()` with a deterministic error message.

2. **Borrow checker invariants (Entropic/Vortex Engine)**
   - Mutable aliasing violations are rejected at compile time.
   - Diagnostics identify collision origins and the collision site.

3. **ABI policy for externs**
   - Extern ABI follows the Fusion ABI policy.
   - Current policy: externs with aggregates are lowered via by‑pointer ABI rewrite (or rejected if disallowed).

4. **Stdlib safety**
   - The stdlib in `stdlib/` and `src/stdlib/` contains **only `.fu` sources**.
   - `dist/lib/fusion/std/src` (target layout, not yet built) will contain `lib.fu` and `main.fu`.
   - Stdlib entry points are `lib.fu` and `main.fu` and are valid Fusion syntax.

5. **Sysroot‑based build resolution**
   - `fusion build` uses sysroot discovery for stdlib/runtime and does **not** consult `Fusion.toml`.

## Explicit Limits (v1.0)

1. **Heap allocation policy**
   - Large arrays are not automatically heap‑allocated unless specified by runtime configuration.
   - Stack allocation thresholds are conservative; oversized arrays may be rejected or abort at runtime.

2. **LTO behavior**
   - Cross‑language LTO is supported only when explicitly enabled and toolchain components match.

3. **Interop boundaries**
   - Non‑Fusion sources (C/C++/JS/Java/etc) are treated as **interop assets** and require explicit wrappers.
   - Interop code is not automatically verified by the borrow checker.

4. **Platform variance**
   - ABI and codegen behavior may differ across OS/arch targets.
   - Windows/MSVC and Linux/Clang toolchains are validated separately.

## Release Condition

A release is considered *secure* only if:
- All build gates pass (see `docs/RELEASE_GATE.md`).
- `scripts/security_gate.sh` exits with status 0.
