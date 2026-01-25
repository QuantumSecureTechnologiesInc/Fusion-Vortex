# Fusion Python Package Manager

**Version:** 0.2.0
**Type:** Tooling
**License:** MIT

## Overview

Fusion Python Package Manager (`interop-python-pkgmgr`) is a utility crate for managing Python dependencies within a Fusion project context. It wraps `pip` and `conda` to ensure reproducible environments.

## Features

- **Dependency Resolution**: Parses `requirements.txt` or `pyproject.toml`
- **Installation**: Installs packages into isolated environments
- **Locking**: Generates lockfiles for Python deps
- **Cross-Platform**: Supports Windows/Linux/macOS

## Usage

```rust
use interop_python_pkgmgr::{PackageManager, Package};

let pm = PackageManager::new("venv");
pm.install(&Package::new("numpy", "1.24"))?;
```text

## Dependencies

- `fusion_core`
- `fusion_std`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)