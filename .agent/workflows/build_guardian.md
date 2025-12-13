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
```

## 2. Formatting Check
Ensure code adheres to style guidelines.
```bash
cargo fmt --all -- --check
```

## 3. Linting
Run Clippy to catch common mistakes and improve Rust code.
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

## 4. Build Verification
Compile the entire workspace to verify correctness.
```bash
cargo build --workspace --all-targets
```

## 5. Test Execution
Run all tests to ensure functionality.
```bash
cargo test --workspace
```

## 6. Benchmark Check
Ensure benchmarks compile (verifying the `[[bench]]` configuration).
```bash
cargo bench --workspace --no-run
```

## 7. Documentation Build
Verify that documentation builds correctly.
```bash
cargo doc --workspace --no-deps
```
