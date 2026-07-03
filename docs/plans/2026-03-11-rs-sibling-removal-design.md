# Rust Sibling Removal Design

> Scope approved by user on 2026-03-11: delete only the paired `.rs` siblings in eight active crates, update broken references if needed, run verification, and report exact results.

## Goal

Remove the remaining live Rust source siblings from the approved active crates so the checked-in source of truth is the `.fu` tree for those modules.

## In Scope

- `crates/analyzer`
- `crates/pkgmgr`
- `crates/projects`
- `crates/toolchain`
- `registry/crates/fusion_runtime_core`
- `registry/crates/fusion_runtime_hal`
- `registry/crates/fusion_runtime_mem_mgr`
- `registry/crates/fusion_runtime_scheduler`

## Out of Scope

- Other crates that still contain `.rs` files
- Generated artefacts under `target/`
- Archive material, examples, or converted reference snapshots
- Large-scale build-system redesign

## Approach

Use a conservative same-path deletion strategy:

1. Identify every `.rs` file in the approved crates that has a same-path `.fu` sibling.
2. Check for direct references that explicitly point at the Rust file paths.
3. Delete only those Rust siblings.
4. Repair any broken references created by the deletions.
5. Verify the result with:
   - a direct file-system check proving the paired `.rs` siblings are gone
   - targeted build or check commands to surface breakage

## Risk

The main risk is build compatibility. Cargo and other tooling may still expect `src/lib.rs`, `main.rs`, or Rust bench entrypoints even when `.fu` siblings exist. If verification fails after deletion, the exact failure output must be reported rather than masked.

## Success Criteria

- No paired `.rs` siblings remain in the eight approved crates.
- Any direct file-path references broken by deletion are fixed.
- Verification output is captured and reported exactly.
