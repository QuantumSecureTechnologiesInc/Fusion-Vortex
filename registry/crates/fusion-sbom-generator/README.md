# fusion-sbom-generator

Software Bill of Materials (SBOM) generator for Fusion projects.

## Features

- Dependency graph analysis
- SPDX and CycloneDX output formats
- Vulnerability scanning integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fusion-sbom-generator = "0.1.0"
```

## Usage

```rust
use fusion_sbom_generator::generate;

fn main() {
    generate();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
