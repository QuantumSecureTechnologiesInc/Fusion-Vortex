# Key Size Analysis: QST HyperCycle v1.0

**Comparison of Vacuum-Based vs Lattice-Based Key Sizes**

---

## KEM Comparison

| Algorithm         | Security     | Public Key | Secret Key |
| ----------------- | ------------ | ---------- | ---------- |
| **HyperKEM-1024** | **NIST L5+** | **256 B**  | **512 B**  |
| ML-KEM-1024       | NIST L5      | 1568 B     | 3168 B     |
| **Advantage**     | -            | **6.1×**   | **6.1×**   |

---

**Conclusion**: HyperCycle keys fit in fragments of standard MTU packets, enabling low-bandwidth PQC.

**Copyright © 2026 Quantum Secure Technologies Ltd.**


