# Fusion Cloud GCP

**Version:** 0.2.0
**Type:** Cloud Adapter
**License:** MIT

## Overview

Fusion Cloud GCP (`cloud-gcp`) provides integration with Google Cloud Platform. It supports Google Cloud Storage, Cloud Functions, and integration with Google's quantum research tools (Cirq interop).

## Features

- **Auth**: Service Account authentication
- **Storage**: GCS bucket management
- **Compute**: Cloud Run / Functions support

## Usage

```rust
use cloud_gcp::{GcpClient, StorageBucket};

let client = GcpClient::from_env().await?;
let bucket = client.bucket("data-lake");
bucket.upload("dataset.csv", data).await?;
```text

## Dependencies

- `fusion_core`
- `google-cloud-storage`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)