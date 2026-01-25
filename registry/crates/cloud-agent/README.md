# Cloud Agent

**Version:** 0.2.0
**Type:** Cloud Infrastructure
**License:** MIT

## Overview

Fusion Cloud Agent (`fusion_q_cloud_agent`) manages the execution of Quantum and Classical jobs across supported cloud providers (AWS, Azure, GCP). It handles authentication, resource provisioning, and job lifecycle management.

## Features

- **Multi-Cloud Support**: Unified interface for AWS Braket, Azure Quantum, etc.
- **Auto-Scaling**: Provisions resources based on queue depth
- **Cost Management**: Enforces budget limits on cloud spending
- **QPU Routing**: Selects optimal QPU backend based on connectivity and availability

## Usage

```rust
use fusion_q_cloud_agent::{CloudAgent, Provider};

let agent = CloudAgent::new(Provider::AWS, "us-east-1");
agent.submit_circuit(quantum_circuit).await?;
```text

## Dependencies

- `fusion_quantum_sdk`
- `fusion_net`
- `reqwest`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)