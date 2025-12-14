# Fusion K8s Operator

**Version:** 0.2.0  
**Type:** Infrastructure  
**License:** MIT

## Overview

The Fusion Kubernetes Operator (`fusion-k8s-operator`) automates the deployment and management of Fusion applications on Kubernetes clusters. It defines Custom Resource Definitions (CRDs) for Fusion Services, Functions, and Quantum Jobs.

## Features

- **CRD Management**: `FusionService`, `FusionFunction`
- **Auto-Scaling**: Horizontal Pod Autoscaling based on Fusion telemetry
- **Sidecar Injection**: Automatically injects Fusion Runtime sidecars
- **Observability**: Exports metrics to Prometheus

## Usage

Deploy the operator to your cluster:

```bash
kubectl apply -f deploy/operator.yaml
```

Define a Fusion Service:

```yaml
apiVersion: fusion.tech/v1
kind: FusionService
metadata:
  name: my-app
spec:
  image: my-app:latest
  replicas: 3
```

## Dependencies

- `kube`
- `k8s-openapi`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
