# Fusion Trusted Anchor

**Version:** 0.2.0
**Type:** Security
**License:** MIT

## Overview

Fusion Trusted Anchor (`fusion_sec_trust_anchor`) manages the hardware Root of Trust for Fusion systems. It interfaces with TPMs and Secure Enclaves to ensure measured boot and platform integrity.

## Features

- **Measured Boot**: Verifies boot chain signatures
- **Attestation**: Generates remote attestation quotes
- **Key Sealing**: Binds cryptographic keys to platform state

## Usage

```rust
use fusion_sec_trust_anchor::{Tpm, PcrBank};

let tpm = Tpm::open()?;
let quote = tpm.quote(PcrBank::Sha256, &[0, 1, 2], nonce)?;
```text

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)