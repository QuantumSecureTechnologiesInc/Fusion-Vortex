# sec-incident-response

Automated incident response and orchestration for security events.

## Features

- Playbook execution
- Alert triage
- Integration with SIEM

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sec-incident-response = "0.1.0"
```

## Usage

```rust
use sec_incident_response::Resolver;

fn main() {
    Resolver::new().run();
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
