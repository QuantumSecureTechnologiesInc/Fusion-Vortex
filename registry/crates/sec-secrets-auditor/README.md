# sec-secrets-auditor

Scanner for detecting hardcoded secrets and credentials.

## Features

- Entropy checking
- Pattern matching for keys
- CI/CD integration

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sec-secrets-auditor = "0.1.0"
```text

## Usage

```rust
use sec_secrets_auditor::scan;

fn main() {
    scan("src/");
}
```text

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.