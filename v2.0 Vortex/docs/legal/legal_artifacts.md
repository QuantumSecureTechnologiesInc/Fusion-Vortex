# Technical Disclosure – HyperCycle Chaos‑Quaternion Cryptography

*Date:* 06 January 2026  
*Classification:* Cryptographic primitives / hardware–software co‑design

This document summarises the inventive concepts underlying HyperCycle Chaos‑Quaternion Cryptography (CQC) and serves as a technical disclosure.  It explains how the system departs from existing lattice‑based post‑quantum schemes by leveraging a physics‑inspired chaotic map and hardware‑coupled topology.

## Problem Statement

Standard post‑quantum cryptography (PQC) algorithms such as ML‑KEM rely on structured lattices and the hardness of the Shortest Vector Problem.  These constructions require large matrix operations and number‑theoretic transforms, leaving them bounded by memory bandwidth and susceptible to breakthroughs in lattice reduction algorithms.  A monoculture of lattice‑based schemes poses systemic risk.

## HyperCycle Approach

HyperCycle CQC introduces a **chaos‑based hard problem**.  Instead of lattice algebra, it uses the non‑linear Heisenberg–Euler map to evolve a bundle of 64 independent 64‑bit trajectories through a chaotic horizon.  The private key consists of the initial states; the public key is the evolved state after a fixed number of iterations.  Because the map exhibits a positive Lyapunov exponent, small perturbations in the initial state lead to exponential divergence, making reverse prediction computationally infeasible.

### Claim A – Heisenberg–Euler Chaos Primitive

The core evolution is defined by

\[
S_{t+1} = S_t \cdot \bigl(1 + \alpha \lvert S_t\rvert^2\bigr) \oplus \text{Coupling}(S_{\text{neighbour}}),
\]

where `Coupling` mixes in the state of a neighbouring trajectory.  The self‑interaction mimics vacuum polarisation, creating a trapdoor function: forward evolution is easy (\(O(N)\)), but inversion requires solving a chaotic system with exponential sensitivity to initial conditions.

### Claim B – Hyper‑Torus Wavefront Topology

The algorithm maps each cryptographic key to a **GPU wavefront** (64 threads on AMD, emulated on 32‑thread NVIDIA warps via double pumping).  Hardware shuffle instructions (`__shfl_xor_sync`, `__shfl_xor`) implement a hyper‑torus topology, coupling trajectories across the wavefront.  Thus the hardware’s geometry becomes part of the cryptographic primitive, binding software logic to the execution substrate.

### Claim C – Double‑Pumped Warp Logic

On hardware with 32‑thread warps, the algorithm emulates a 64‑mode topology by having each thread track two trajectories.  Vertical mixing between the two states allows the algorithm to preserve its 64‑node hyper‑torus structure without sacrificing compatibility with narrower warps.

## Embodiments

HyperCycle CQC is implemented in the `hc_core` software stack.  The NVIDIA embodiment (`hc_vacuum_gpu.cu`) and AMD embodiment (`hc_vacuum_amd.hip.cpp`) wrap the chaos primitive in GPU kernels, allocate pinned memory for zero‑copy host/device transfers and manage persistent contexts for throughput.

## Conclusion

HyperCycle replaces lattice‑based number theory with a chaotic time‑evolution problem and binds the cryptographic logic to the underlying hardware topology.  This constitutes a novel and non‑obvious solution to post‑quantum key generation and forms the basis for further patents and research.

