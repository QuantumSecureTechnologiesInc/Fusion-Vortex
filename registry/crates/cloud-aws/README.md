# Fusion Cloud AWS

**Version:** 0.2.0  
**Type:** Cloud Adapter  
**License:** MIT

## Overview

Fusion Cloud AWS (`cloud-aws`) provides native integration with Amazon Web Services. It allows Fusion applications to interact with AWS services like S3 (for storage), Lambda (for compute), and Braket (for quantum kernels).

## Features

- **Auth**: IAM role and credential management
- **Storage**: S3 bucket operations
- **Compute**: Lambda function invocation
- **Quantum**: AWS Braket backend for QPU access

## Usage

```rust
use cloud_aws::{AwsClient, Service};

let client = AwsClient::from_env().await?;
client.invoke(Service::Lambda, "my-function", payload).await?;
```

## Dependencies

- `fusion_core`
- `aws-sdk-s3`
- `aws-sdk-lambda`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
