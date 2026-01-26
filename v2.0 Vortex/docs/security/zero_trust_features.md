# Zero‑Trust Design

HyperCycle was designed with a zero‑trust security model in mind.  In such a model, the system assumes that any component may be compromised and therefore never relies on implicit trust.  This document highlights the ways in which the library embraces zero‑trust principles.

## No Persistent Secrets

The library does not store persistent secrets within its static data structures.  All sensitive material (seeds, intermediate states and keys) is passed explicitly via function arguments and stored on the stack or in caller‑supplied buffers.  GPU contexts encapsulate only runtime resources; they do not retain keys between calls.

## Pinned Memory for Key Material

When running on a GPU, host memory used for inputs and outputs can be pinned via `alloc_pinned()`.  Pinned memory remains resident in RAM and is not swapped to disk by the operating system, reducing the risk of sensitive data being written to persistent storage.  The backends automatically detect whether user buffers are pinned and choose the appropriate transfer mode.

## Constant‑Time Arithmetic

All cryptographic operations avoid data‑dependent branches in their inner loops.  The chaos map uses integer arithmetic and a fixed number of iterations.  The blinding mechanism executes the same number of operations whether or not masking is enabled, interleaving the main and mask trajectories to hide latency.  These properties mitigate timing and power side‑channel attacks.

## Health Monitoring and Self‑Tests

The Hamiltonian vacuum engine performs live
**Repetition Count** and **Adaptive Proportion** tests on its output to
detect entropy collapses.  On initialisation it executes a
1 024‑cycle power‑on self‑test.  If any check fails, the engine aborts
generation and enters the recovery routine described below.

## Self‑Healing Routines

Should the health monitor detect a failure, the engine attempts to restore
chaos through a multi‑tiered recovery: (1) **Perturbation** by adding a
large constant to the momentum state, (2) **Hardware Reseed** by XORing
hardware RNG bytes into the position state, and (3) **Hard Reset** by
re‑initialising the engine and rerunning the warm‑up sequence.  Only if all
tiers fail does the API return an error to the caller.

## Explicit Clearing of Buffers

Callers are encouraged to clear sensitive buffers after use.  The library itself zeroes out internal telemetry and context structures when freeing resources.  While C does not provide portable guarantees for securely zeroing memory, you can call `memset_s()` or similar functions on the output buffers when they are no longer needed.

## Minimal Privileges

HyperCycle does not require elevated privileges.  It relies only on standard CUDA/HIP runtime APIs and POSIX functions.  There is no dependency on kernel modules beyond those provided by your GPU drivers.

## Further Hardening

Although the library embodies many zero‑trust principles, additional measures may be warranted in a production deployment:

- **Entropy Sources** – The library now ships with a Hamiltonian vacuum engine that provides its own entropy through chaotic time evolution, health tests and SHA3 conditioning.  Should your threat model require additional diversity, you may supplement this with hardware TRNGs or `/dev/urandom`.

- **Ed25519 Hooks** – The first‑party Ed25519 implementation includes
  configuration fields (`rng_hook` and `entropy_mixer`) that allow you to
  mix vacuum entropy into key generation, signing and verification.  This
  bridges classical and quantum‑resistant algorithms without trusting
  external RNGs.
- **Code Auditing** – Perform static and dynamic analysis to identify potential memory safety issues, particularly in GPU kernels.
- **Formal Verification** – Pursue formal proofs of security for the PQC primitives to substantiate resistance to quantum adversaries.


