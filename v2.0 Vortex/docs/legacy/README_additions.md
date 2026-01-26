# HyperCycle Full Library Additions

This directory combines the original **HyperCycle v1.1 Origin** source tree with the **HyperCycle PQC Add‑ons** developed separately.  Nothing has been removed from the original library; the additional modules live under `hypercycle_pqc_addons` and provide:

* A Hamiltonian vacuum entropy engine with NIST SP 800‑90B health tests and self‑healing.
* A unified GPU/CPU loader that supports pinned memory detection, telemetry, and introspection.
* Quaternion‑based PQC primitives and a secure Ed25519 signature wrapper powered by OpenSSL.
* Comprehensive documentation for the new modules in `hypercycle_pqc_addons/docs`.

These add‑ons are independent from the original HyperCycle v1.1 code and do not alter any of its algorithms (e.g., OGA‑KEM, lattice mode).  They are provided for those who wish to experiment with advanced entropy sources and post‑quantum components alongside the existing features.

To build the original library, follow the instructions in the root `README.md`.  To build and use the PQC add‑ons, consult the `hypercycle_pqc_addons/README.md` file.

