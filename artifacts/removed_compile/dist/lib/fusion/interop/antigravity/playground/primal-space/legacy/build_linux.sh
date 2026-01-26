#!/bin/bash
# Linux build script for NeuralSeal/CEMQC
#
# This script creates a separate build directory, configures CMake
# with self‑tests enabled and builds the static library and test
# runner using all available CPU cores.  Pass "debug" as the first
# argument to produce a debug build instead of release.

set -euo pipefail

BUILD=${1:-release}
BUILD_DIR="build-linux"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ "$BUILD" = "debug" ]; then
  cmake -DCMAKE_BUILD_TYPE=Debug -DCEMQC_SELFTEST=ON ..
else
  cmake -DCMAKE_BUILD_TYPE=Release -DCEMQC_SELFTEST=ON ..
fi

# Determine number of parallel jobs.  Fallback to 1 if `nproc` is unavailable.
if command -v nproc >/dev/null 2>&1; then
  JOBS=$(nproc)
else
  JOBS=1
fi

cmake --build . -j "$JOBS"

echo
echo "✅ Linux build complete! Run ./cemqc_selftest_runner to execute self‑tests."