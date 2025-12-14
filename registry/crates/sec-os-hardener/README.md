# sec-os-hardener

System hardening utilities for Fusion deployment environments.

## Features

- Kernel auditing
- Service configuration
- Compliance checking

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sec-os-hardener = "0.1.0"
```

## Usage

```rust
use sec_os_hardener::Auditor;

fn main() {
    Auditor::check();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
