# Fusion Tester

**Version:** Workspace
**Type:** Dev Tool
**License:** MIT

## Overview

Fusion Tester (`fusion-tester`) is the unified testing framework for the Fusion language. It supports unit tests, integration tests, and end-to-end compiler verification.

## Features

- **Test Discovery**: Automatically finds `#[test]` functions
- **Assertions**: `assert_eq!`, `assert_throws!`, etc.
- **Test Doubles**: Built-in test-double utilities
- **Reporting**: Colored output and JUnit XML export

## Usage

```bash
fusion test
```text

Or programmatically:

```rust
use fusion_tester::Runner;

let runner = Runner::new();
runner.run_tests("src/")?;
```text

## Dependencies

- `fusion-core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
