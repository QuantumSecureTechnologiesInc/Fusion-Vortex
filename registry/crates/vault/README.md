# Fusion Vault

**Version:** 0.2.0  
**Type:** Security  
**License:** MIT

## Overview

Fusion Vault (`fusion_vault`) is a secure credential storage system. It abstracts over OS-level keychains (Windows Credential Manager, macOS Keychain, Linux Secret Service) to safely store API keys and tokens.

## Features

- **Encryption**: Data is encrypted at rest using system-tied keys
- **Storage**: Key/Value interface for secrets
- **Isolation**: Namespace isolation for different apps

## Usage

```rust
use fusion_vault::Vault;

let vault = Vault::open("my-app")?;
vault.set("api_key", "secret_value")?;
let key = vault.get("api_key")?;
```

## Dependencies

- `fusion_core`
- `fusion_std`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
