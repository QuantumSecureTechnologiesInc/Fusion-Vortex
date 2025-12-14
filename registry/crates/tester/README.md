# Fusion Tester

**Version:** Workspace  
**Type:** Dev Tool  
**License:** MIT

## Overview

Fusion Tester (`fusion-tester`) is the unified testing framework for the Fusion language. It supports unit tests, integration tests, and end-to-end compiler verification.

## Features

- **Test Discovery**: Automatically finds `#[test]` functions
- **Assertions**: `assert_eq!`, `assert_throws!`, etc.
- **Mocking**: Built-in mocking utilities
- **Reporting**: Colored output and JUnit XML export

## Usage

```bash
fusion test
```

Or programmatically:

```rust
use fusion_tester::Runner;

let runner = Runner::new();
runner.run_tests("src/")?;
```

## Dependencies

- `fusion-core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
