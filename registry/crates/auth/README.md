# Fusion Auth

**Version:** 0.2.0
**Type:** Security Library
**License:** MIT

## Overview

Fusion Auth (`fusion_auth`) provides core authentication and authorization utilities for the Fusion ecosystem. It handles token generation, validation, and identity management.

## Features

- **Token Management**: JWT-compatible token generation and parsing
- **Encoding**: Safe Base64 handling
- **Identity**: User and Role abstractions
- **Serialization**: Secure serialization of auth contexts

## Usage

```rust
use fusion_auth::{AuthContext, Token};

// Create a context
let ctx = AuthContext::new("user_123", vec!["admin"]);

// Generate token
let token = Token::generate(&ctx, "secret_key")?;
```text

## Dependencies

- `fusion_core`
- `base64`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)