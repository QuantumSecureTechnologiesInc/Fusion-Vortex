# Hamiltonian Vacuum Engine

The **vacuum engine** is a novel entropy generator integrated into HyperCycle v1.7.2.  It replaces the simplistic RNG placeholders used in earlier versions with a mathematically rigorous Hamiltonian time‑evolution simulator.  The design draws upon the integration documents and implements several advanced features to guarantee high‑quality randomness even under extreme adversarial conditions.

## Design Overview

At its core the engine simulates eight chaotic trajectories in parallel using AVX‑512 instructions.  Each trajectory evolves under a **Kick–Drift** symplectic integrator: momentum is first updated according to a non‑linear force (computed via a vectorised skew‑tent map), then position is updated based on the new momentum.  A tiny jitter derived from the CPU timestamp counter is XORed into the momentum every step to prevent the system from settling into a periodic orbit.

To ensure compliance with **NIST SP 800‑90B**, the engine implements two live health tests:

1. **Repetition Count Test (RCT)** – tracks how many times the same 64‑bit sample has been produced consecutively and triggers a failure if the count exceeds a threshold.
2. **Adaptive Proportion Test (APT)** – maintains a sliding window of recent samples and detects if any value appears too frequently.  If the number of matches in the window exceeds a cutoff, the engine signals a collapse.

Raw chaotic output (512 bits per call) is **conditioned via SHA3‑256**, producing a 32‑byte seed suitable for PQC algorithms such as Kyber and Dilithium.  Between calls the momentum register is incremented to provide forward secrecy.

## Self‑Healing Mechanisms

Should either health test detect a collapse, the engine attempts to restore chaos automatically through a **multi‑tiered recovery strategy**:

1. **Perturbation** – Adds a large constant (the golden ratio) to all momentum lanes, nudging the system off any periodic orbit.
2. **Hardware Reseed** – XORs fresh noise from the CPU’s hardware RNG (via `RAND_priv_bytes`) into the position state.
3. **Hard Reset** – Wipes the health monitor and re‑runs the 1,024‑cycle warm‑up sequence to re‑initialise the system.

If recovery fails after all tiers, the engine returns an error and must be re‑initialised.

## Startup Warm‑Up

Upon initialisation the engine performs at least **1,024 evolution steps** before releasing any entropy.  Each step is health‑checked to verify that the initial seed has entered a high‑entropy orbit.  If any check fails, initialisation fails.

## API Summary

The vacuum engine is exposed via the header `hc_vacuum_engine.h`.  It defines:

- `hc_vacuum_init_context(&ctx, &cfg)` – initialises a new context, deriving a seed from `cfg.device_id`, the allocator address and the current time.
- `hc_vacuum_generate_seed(ctx, out_seed)` – produces a 32‑byte seed.  Returns an error if a health failure occurs.
- `hc_vacuum_generate_seed_safe(ctx, out_seed)` – as above but attempts recovery on failure.  Returns an error only if all recovery tiers fail.
- `hc_vacuum_get_telemetry(ctx, &telemetry)` – returns counters such as total bytes generated and the duration of the last call.
- `hc_vacuum_free_context(ctx)` – securely wipes and frees the context.

All functions are thread‑safe: internal state is protected by a mutex.  Sensitive data is wiped from memory on teardown using a volatile overwrite and an assembly barrier to prevent compiler optimisation.

## Integration Notes

* The engine requires AVX‑512F support and links against OpenSSL’s libcrypto for SHA3 and hardware random bytes.  When compiling, add `-mavx512f` and link with `-lcrypto`.
* You can use the vacuum engine independently of the GPU loader.  It is ideal for generating PQC seeds in environments where GPU acceleration is not available or when you need deterministic reproducibility across platforms.
* The engine is fully deterministic given the initial seed.  To maximise entropy in long‑running applications you can periodically reseed via `hc_vacuum_generate_seed_safe` or call `hc_vacuum_init_context` anew.


