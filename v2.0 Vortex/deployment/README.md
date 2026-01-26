# HyperCycle Vortex v2.0.2 - Deployment Guide

## Overview

This directory contains multiple deployment options for HyperCycle Vortex v2.0.2, the most advanced post-quantum cryptographic entropy engine.

## Deployment Options

### 1. Docker Deployment

**Best for**: Local development, testing, containerized environments

- **Location**: `docker/`
- **Quick Start**: `docker-compose up -d`
- **Documentation**: [docker/README.md](docker/README.md)

### 2. Kubernetes Deployment

**Best for**: Production, cloud-native, scalable deployments

- **Location**: `kubernetes/`
- **Quick Start**: `kubectl apply -k kubernetes/`
- **Documentation**: [kubernetes/README.md](kubernetes/README.md)

### 3. Local Development

**Best for**: Development, debugging, testing

- **Location**: `local/`
- **Quick Start**:
  - Windows: `.\local\setup.ps1`
  - Linux/macOS: `./local/setup.sh`
- **Documentation**: [local/README.md](local/README.md)

### 4. Production Deployment

**Best for**: Enterprise production environments

- **Location**: `production/`
- **Quick Start**: See [production/README.md](production/README.md)
- **Security**: [production/security-hardening.md](production/security-hardening.md)

### 5. CI/CD Pipelines

**Best for**: Automated testing and deployment

- **Location**: `ci-cd/`
- **Platforms**: GitHub Actions, GitLab CI, Jenkins
- **Documentation**: [ci-cd/README.md](ci-cd/README.md)

### 6. Monitoring & Observability

**Best for**: Production monitoring, metrics, alerting

- **Location**: `monitoring/`
- **Tools**: Prometheus, Grafana
- **Documentation**: [monitoring/README.md](monitoring/README.md)

## Quick Comparison

| Deployment Type | Setup Time | Complexity | Best For |
|-----------------|------------|------------|----------|
| **Docker** | 5 minutes | Low | Development, Testing |
| **Kubernetes** | 15 minutes | Medium | Production, Cloud |
| **Local** | 10 minutes | Low | Development |
| **Production** | 30 minutes | High | Enterprise |
| **CI/CD** | 20 minutes | Medium | Automation |

## Prerequisites

### All Deployments

- CMake 3.21+
- GCC 9+ or Clang 9+ or MSVC 2019+
- AVX-512 capable CPU (recommended)

### Docker

- Docker 20.10+
- Docker Compose 2.0+

### Kubernetes

- Kubernetes 1.21+
- kubectl configured
- Helm 3.0+ (optional)

### Production

- OpenSSL 1.1.1+ (optional, has internal fallback)
- CUDA Toolkit 11.0+ (optional, for GPU acceleration)

## Security Considerations

All deployment options include:

- ✅ Stack protection (`-fstack-protector-strong`)
- ✅ Buffer overflow detection (`_FORTIFY_SOURCE=2`)
- ✅ Format security warnings
- ✅ PIE/ASLR support
- ✅ Read-only relocations

See [production/security-hardening.md](production/security-hardening.md) for additional hardening.

## Support

- **Documentation**: [../README.md](../README.md)
- **Issues**: Report via project issue tracker
- **Security**: See SECURITY.md for vulnerability reporting

## Version

- **HyperCycle Version**: 2.0.2
- **Deployment Guide Version**: 1.0
- **Last Updated**: 2026-01-25
