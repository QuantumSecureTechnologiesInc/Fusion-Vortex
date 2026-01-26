#!/bin/bash
set -e
cd "/mnt/c/Users/Matth/.gemini/antigravity/scratch/HyperCycle/v2.0 Vortex/"

echo "[1/3] Compiling Vortex Core..."
# Compile all C files to objects first
mkdir -p build_objs
SOURCES=(
  "src/vortex/weave_kem.c"
  "src/vortex/weave_sig.c"
  "src/vortex/ml_kem.c"
  "src/vortex/ml_dsa_87.c"
  "src/vortex/hc_oga_kem.c"
  "src/vortex/hc_hybrid_x25519.c"
  "src/vortex/cemqc.c"
  "src/vortex/sha3.c"
  "src/vortex/system_entropy.c"
  "src/vortex/hc_quaternion.c"
  "src/vortex/hc_octonion.c"
  "src/vortex/hc_octonion_simd.c"
  "src/vortex/hc_vacuum.c"
  "src/vortex/hc_vacuum_avx512.c"
  "src/vortex/hc_vacuum_jitter.c"
  "src/vortex/hypercycle_algorithms.c"
  "src/vortex/hc_cpu_features.c"
  "src/vortex/hc_health_tests.c"
  "src/vortex/hc_core.c"
  "src/vortex/hc_gpu_universal.c"
  "src/vortex/hc_fast_validation.c"
  "src/vortex/hc_sbox16.c"
  "src/vortex/hc_telemetry.c"
  "src/vortex/batch.c"
  "src/vortex/hc_cbom.c"
  "src/vortex/fixed_point.c"
  "src/vortex/hc_constant_time.c"
  "src/vortex/hc_introspection.c"
  "src/vortex/hc_key_rotation.c"
  "src/vortex/hc_policy_engine.c"
  "src/vortex/hc_risk_score.c"
  "src/vortex/hc_secure_enclave.c"
  "src/vortex/hc_secure_memory.c"
  "src/vortex/hc_zero_trust.c"
  "src/vortex/hypercycle_consciousness.c"
  "src/vortex/hypercycle_core.c"
  "src/vortex/hypercycle_genesis.c"
  "src/vortex/hypercycle_quantum_accel.c"
  "src/vortex/hypercycle_temporal.c"
  "src/vortex/hypercycle_ultra_optimizer.c"
  "src/vortex/ns_entropy_pool.c"
  "src/vortex/hc_final.c"
  "src/hc_vacuum_engine.c"
)

for src in "${SOURCES[@]}"; do
  obj="build_objs/$(basename ${src%.c}.o)"
  echo "  CC $src -> $obj"
  gcc -O3 -march=native \
    -Iinclude \
    -Iinclude/vortex/public \
    -Iinclude/vortex/internal \
    -Iinclude/vortex \
    -Isrc/include \
    -c "$src" -o "$obj"
done

echo "[2/3] Compiling Benchmark..."
gcc -O3 -march=native \
  -Iinclude \
  -Iinclude/vortex/public \
  -Iinclude/vortex/internal \
  -Iinclude/vortex \
  -Isrc/include \
  -c tests/benchmark_all_algorithms.c -o build_objs/benchmark_all_algorithms.o

echo "[3/3] Linking..."
# Link all objects together. Ordering them so telemetry is provided.
gcc build_objs/*.o \
  -lcrypto -lm -lpthread -ldl \
  -o benchmark_all_algorithms

echo "Done! Binary: ./benchmark_all_algorithms"
