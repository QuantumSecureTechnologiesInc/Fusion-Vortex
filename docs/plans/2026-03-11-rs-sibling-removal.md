# Rust Sibling Removal Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove only the approved paired `.rs` siblings from eight active crates, repair any broken references, and verify the result with exact command output.

**Architecture:** The work is a constrained source-tree cleanup rather than a feature change. The implementation deletes only same-path Rust siblings where a `.fu` counterpart already exists, then validates both file removal and workspace impact.

**Tech Stack:** PowerShell, `rg`, `git`, `apply_patch`, Cargo workspace checks

---

### Task 1: Record the approved scope

**Files:**
- Create: `docs/plans/2026-03-11-rs-sibling-removal-design.md`
- Create: `docs/plans/2026-03-11-rs-sibling-removal.md`

**Step 1: Write the design and plan documents**

Capture the approved crate list, the conservative deletion rule, and the verification strategy.

**Step 2: Verify the files exist**

Run: `Get-ChildItem docs/plans`
Expected: both plan files are listed

### Task 2: Prove the precondition fails

**Files:**
- Inspect only

**Step 1: Run a red check for paired Rust siblings**

Run a command that enumerates `.rs` files in the approved crates and fails if none exist.

**Step 2: Verify the command fails for the expected reason**

Expected: the command reports that paired `.rs` files are still present before deletion.

### Task 3: Inspect for direct path references

**Files:**
- Modify only if needed after inspection

**Step 1: Search the approved crates for references to `.rs` file paths**

Run: `rg -n "\.rs\b|path = .*\.rs|src/.*\.rs|benches/.*\.rs" ...`
Expected: either no direct path dependencies or a small list to fix

**Step 2: Record any required edits**

Only edit files if a direct path reference would break after deletion.

### Task 4: Delete the approved paired Rust siblings

**Files:**
- Delete: `crates/analyzer/src/lib.rs`
- Delete: `crates/pkgmgr/src/lib.rs`
- Delete: `crates/projects/src/db.rs`
- Delete: `crates/projects/src/lib.rs`
- Delete: `crates/projects/src/state.rs`
- Delete: `crates/toolchain/src/builder.rs`
- Delete: `crates/toolchain/src/lib.rs`
- Delete: `crates/toolchain/src/project.rs`
- Delete: `crates/toolchain/src/runner.rs`
- Delete: `crates/toolchain/src/sysroot.rs`
- Delete: `registry/crates/fusion_runtime_core/benches/runtime_benchmarks.rs`
- Delete: `registry/crates/fusion_runtime_core/src/collective_comms.rs`
- Delete: `registry/crates/fusion_runtime_core/src/config.rs`
- Delete: `registry/crates/fusion_runtime_core/src/device_memory.rs`
- Delete: `registry/crates/fusion_runtime_core/src/event_poller.rs`
- Delete: `registry/crates/fusion_runtime_core/src/executor.rs`
- Delete: `registry/crates/fusion_runtime_core/src/fiber.rs`
- Delete: `registry/crates/fusion_runtime_core/src/lib.rs`
- Delete: `registry/crates/fusion_runtime_core/src/llm.rs`
- Delete: `registry/crates/fusion_runtime_core/src/qpu_sequencer.rs`
- Delete: `registry/crates/fusion_runtime_core/src/shared_memory.rs`
- Delete: `registry/crates/fusion_runtime_core/src/task.rs`
- Delete: `registry/crates/fusion_runtime_core/src/timer.rs`
- Delete: `registry/crates/fusion_runtime_hal/src/gpu.rs`
- Delete: `registry/crates/fusion_runtime_hal/src/lib.rs`
- Delete: `registry/crates/fusion_runtime_hal/src/network.rs`
- Delete: `registry/crates/fusion_runtime_hal/src/qpu.rs`
- Delete: `registry/crates/fusion_runtime_mem_mgr/src/lib.rs`
- Delete: `registry/crates/fusion_runtime_scheduler/src/lib.rs`
- Delete: `registry/crates/fusion_runtime_scheduler/src/task.rs`
- Delete: `registry/crates/fusion_runtime_scheduler/src/vlc.rs`

**Step 1: Delete the files with `apply_patch`**

Remove only the same-path `.rs` siblings listed above.

**Step 2: Re-run the paired-sibling check**

Expected: no `.rs` siblings remain in the approved crates.

### Task 5: Repair broken references if any surface

**Files:**
- Modify only if verification exposes a direct reference problem

**Step 1: Re-run the direct-reference search**

Expected: no surviving references to the deleted file paths.

**Step 2: Patch only the necessary references**

Keep edits minimal and local to the failure.

### Task 6: Run verification and report exact results

**Files:**
- Inspect only

**Step 1: Verify deletion from the filesystem**

Run a command that lists `.rs` files remaining in the approved crates.
Expected: zero results.

**Step 2: Verify the git diff**

Run: `git status --short -- <approved paths>`
Expected: deleted `.rs` files and any deliberate reference fixes.

**Step 3: Run build validation**

Run targeted `cargo check` commands or the relevant workspace check commands and capture the exact failures or passes.

**Step 4: Report exact output**

Do not infer success. Report the command results exactly, including any build breakage caused by removing Rust entrypoints.
