# Fusion Cloud Azure

**Version:** 0.2.0
**Type:** Cloud Adapter
**License:** MIT

## Overview

Fusion Cloud Azure (`cloud-azure`) provides native integration with Microsoft Azure. It facilitates interaction with Azure Blob Storage, Azure Functions, and Azure Quantum.

## Features

- **Identity**: Azure AD authentication
- **Storage**: Blob operations
- **Quantum**: Azure Quantum backend provider

## Usage

```rust
use cloud_azure::{AzureClient, QuantumJob};

let client = AzureClient::from_env().await?;
// Submit quantum circuit to IonQ via Azure
client.submit_job(QuantumJob::new(circuit)).await?;
```text

## Dependencies

- `fusion_core`
- `azure_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)