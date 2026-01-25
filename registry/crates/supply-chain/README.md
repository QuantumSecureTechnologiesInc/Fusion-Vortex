# Fusion Supply Chain

**Version:** 0.2.0
**Type:** Security Tool
**License:** MIT

## Overview

Fusion Supply Chain (`fusion_sec_supply_chain`) is a security toolkit for securing the software supply chain. It generates Software Bill of Materials (SBOMs), verifies artifact signatures, and analyzes dependency trees for risks.

## Features

- **SBOM Generation**: Output in SPDX and CycloneDX formats
- **Verification**: Check signatures against trusted keys
- **Integrity**: Hash verification for all dependencies
- **Policy**: Enforce allowed licenses and vendors

## Usage

```rust
use fusion_sec_supply_chain::{SbomGenerator, Format};

let generator = SbomGenerator::new(".");
let sbom = generator.generate(Format::CycloneDX)?;
std::fs::write("sbom.json", sbom)?;
```text

## Dependencies

- `fusion_core`
- `sha2`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)