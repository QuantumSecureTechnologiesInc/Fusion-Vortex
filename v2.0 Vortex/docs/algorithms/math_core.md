# Mathematical Core

At the heart of HyperCycle lies a *fixed‑point quaternion chaos map* that turns simple 64‑bit seeds into 256‑bit pseudorandom outputs.  The design draws from the Heisenberg–Euler effective Lagrangian and has been tuned to resist quantum attacks.  This document summarises the core functions and explains the rationale behind their implementation.

## Quaternion Representation

All state is represented as a 128‑bit quaternion `hc_quat_t` consisting of four 64‑bit signed integers `(w,x,y,z)`.  The components are stored in fixed‑point form with a scaling factor of 10⁹ (`HC_SCALE`).  Using integers rather than floats eliminates rounding errors across platforms and simplifies constant‑time implementations.

The structure is aligned to 32 bytes to encourage compilers to generate vectorised load/store instructions.  On CPUs supporting AVX‑512 it also enables the use of integer fused multiply–add instructions (IFMA) for further acceleration.

## Chaos Map

The *Heisenberg–Euler chaos map* evolves a quaternion in three stages per iteration:

1. **Field strength** – Reduce each component by dividing by 1000 and compute the magnitude squared.  This reduction prevents overflow when multiplying large values.
2. **Non‑linear scaling** – Compute the scaling factor \(1 + \alpha \cdot |q|^2\).  The parameter \(\alpha\) is scaled by 10⁹ and chosen so that trajectories exhibit sensitive dependence on initial conditions while maintaining numeric stability.
3. **Symplectic mix** – Apply a simple rotation to couple the real and imaginary parts, spreading entropy across all components without branching.

Each call to `hc_chaos_map_step()` performs one such iteration.  A complete key generation uses 47 iterations (`HC_CYCLES`), a value chosen experimentally to balance diffusion and throughput.

## Key Generation

`hc_generate_single_key()` produces a 256‑bit key from a base seed, an optional blinding seed and an index.  The function supports two execution paths:

* **Fast path** – When the blinding seed is zero, the function initialises the quaternion from `seed_base ^ idx` and iterates the chaos map.  The final state is reduced to four 64‑bit words by combining components with XORs and additions.  This path contains no conditional branches inside the hot loop, aiding compiler vectorisation.
* **Secure path** – When the blinding seed is non‑zero, the function initialises a second quaternion using `blinding_seed ^ (idx ⊕ 0xAAAAAAAA55555555)`.  Both quaternions are iterated in lockstep.  At extraction time the outputs of the two trajectories are XORed together, producing a masked key.  Interleaving the main and mask trajectories hides the latency of divisions and prevents timing and power analysis from distinguishing the two cases.

Throughout the implementation we avoid data‑dependent branches and use 64‑bit arithmetic exclusively.  These properties make the core suitable for deployment in a zero‑trust environment where side‑channel resilience is paramount.

## AVX‑512 Optimisation

On x86 CPUs with AVX‑512IFMA support, the math core can take advantage of fused multiply–add instructions to speed up the non‑linear scaling step.  The library exposes the `HC_FLAG_OPT_AVX512_IFMA` flag; when set and supported, the CPU backend dispatches to a specialised routine defined in `hc_math_avx512.h`.  This optimisation reduces the number of integer multiplications per chaos step and improves throughput without altering the mathematical identity of the output.


