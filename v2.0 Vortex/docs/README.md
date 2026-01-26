# HyperCycle PQC Documentation

This documentation folder contains design notes, algorithmic explanations, benchmark guides and legal disclosures for the HyperCycle PQC library.  Documents are organised by topic to aid navigation.

## Contents

The documentation is divided into several topical folders.  Use the following table as a guide:

| Folder | Purpose |
| --- | --- |
| **algorithms** | Detailed explanations of the mathematical foundations behind HyperCycle.  This includes the fixed‑point chaos map (`math_core.md`), quaternion arithmetic and sponge constructions (`math_boost.md`), the Hamiltonian vacuum engine (`vacuum_engine.md`), and the post‑quantum cryptographic schemes built atop these primitives (`pqc_boost.md`).  The algorithms folder also references the octonion‑based **O‑GA‑KEM** and lattice engines preserved from the origin release.  A high‑level optimisation guide summarises the acceleration techniques extracted from the integration packages. |
| **api** | Descriptions of the public interfaces exposed by the library.  In particular, `gpu_universal.md` outlines the universal GPU abstraction layer, covering contexts, configuration structures, capability queries, memory management and telemetry, while `hc_ed25519.md` documents the built‑in Ed25519 interface and its entropy hooks. |
| **benchmarks** | Instructions and source code for verifying optimisation claims and measuring performance on various hardware.  Use these documents to reproduce the speed‑up factors observed during development. |
| **legal** | Disclosure of the intellectual property behind HyperCycle’s chaos‑based cryptography, including novelty claims and a technical summary. |
| **security** | Guidance on secure deployment practices.  A short note on “zero trust” highlights how the library avoids persistent state, emphasises constant‑time arithmetic and describes the FIPS‑compliant self‑tests and multi‑tier self‑healing implemented in the Hamiltonian vacuum engine. |
| **tests** | Sample programs demonstrating correct usage of the library and verifying key features such as blinding, batch generation, Ed25519 signatures and the Hamiltonian engine’s health monitoring.  Some legacy tests from the origin release remain available under `tests/origin`. |

For a quick start with the API itself see the top‑level `README.md` in the project root.  The documents here provide deeper insight into the underlying science and the rationale behind key design decisions.

