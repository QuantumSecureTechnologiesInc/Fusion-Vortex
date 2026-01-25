# Security Architecture

**Dataset Category**: Architecture
**Training Level**: Advanced
**Last Updated**: December 2025 (v1.0.0)

---

## Overview

Fusion security uses a hybrid classical/PQC stack and Sentinel TriBrid for runtime anomaly detection.

## Components

- PQC stack: ML‑KEM, ML‑DSA, SPHINCS+
- Sentinel TriBrid: CPU/Net/Thermal heuristic isolation
- Secure networking: hybrid TLS + key rotation

## References

- docs/guides/Technical_Sheet.md