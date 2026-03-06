# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| v2.0    | :white_check_mark: |
| v1.x    | :x:                |

## Memory Safety

Fusion v2.0 Vortex guarantees memory safety through its ownership system and compile-time borrow checker, eliminating common vulnerabilities like use-after-free and buffer overflows.

## Secure Networking

The `stdlib/net` and `stdlib/http` modules provide robust interfaces for network communication.

- **TLS/SSL**: Integration with native system TLS libraries is planned for future releases.
- **Input Validation**: All network inputs should be validated using `stdlib/json` parsing or custom validation logic.

## Reporting a Vulnerability

Please report vulnerabilities to `security@fusion-lang.org`. We aim to acknowledge within 24 hours.
