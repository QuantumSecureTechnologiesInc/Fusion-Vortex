---
description: BuildGuardian Integrity Workflow
---

# BuildGuardian Workflow

This workflow executes the standard build verification process for the Fusion project.

## 1. Environment Check

Check that the environment is correctly set up with the required toolchains.

```bash
rustc --version
cargo --version
```text

## 2. Formatting Check

Ensure code adheres to style guidelines.

```bash
cargo fmt --all -- --check
```text

## 3. Linting

Run Clippy to catch common mistakes and improve Rust code.

```bash
cargo clippy --workspace --all-targets -- -D warnings
```text

## 4. Build Verification

Compile the entire workspace to verify correctness.

```bash
cargo build --workspace --all-targets
```text

## 5. Test Execution

Run all tests to ensure functionality.

```bash
cargo test --workspace
```text

## 6. Benchmark Check

Ensure benchmarks compile (verifying the `[[bench]]` configuration).

```bash
cargo bench --workspace --no-run
```text

## 7. Documentation Build

Verify that documentation builds correctly.

```bash
cargo doc --workspace --no-deps
```text