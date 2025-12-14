# Fusion Version

**Version:** 0.2.0  
**Type:** Utility  
**License:** MIT

## Overview

Fusion Version (`fusion_version`) handles semantic versioning logic for the Fusion toolchain. It provides types for parsing, comparing, and incrementing versions according to SemVer 2.0.

## Features

- **SemVer Parsing**: Strict adherence to spec
- **Constraint Solving**: `^1.0`, `~1.2` logic
- **Serialization**: Serde integration

## Usage

```rust
use fusion_version::{Version, Requirement};

let v = Version::parse("1.2.3")?;
let req = Requirement::parse("^1.0")?;
assert!(req.matches(&v));
```

## Dependencies

- `semver`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
