# sbom-generator

Tool for generating Software Bill of Materials (SBOM) for compliance.

## Features

- Dependency tree traversal
- License detection
- SPDX/CycloneDX output

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sbom-generator = "0.1.0"
```text

## Usage

```rust
use sbom_generator::generate;

fn main() {
    generate();
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.