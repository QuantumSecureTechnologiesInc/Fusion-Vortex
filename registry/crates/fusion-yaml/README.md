# Fusion YAML

**Version:** 0.2.0  
**Type:** Data Format  
**License:** MIT

## Overview

Fusion YAML (`fusion-yaml`) is the standard YAML library for Fusion. It provides strictly typed parsing and serialization, optimized for configuration files and Kubernetes-style manifests.

## Features

- **Strict Parsing**: Enforces schema correctness clearly
- **Multi-Document**: Supports multi-document YAML files (streams)
- **Anchor Resolution**: Correctly handles YAML anchors and aliases
- **Preservation**: Can preserve comments and formatting during round-trip

## Usage

```rust
use fusion_yaml::{to_string, from_str};

let config = MyConfig::default();
let yaml = to_string(&config)?;

println!("{}", yaml);
```

## Dependencies

- `fusion_core`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
