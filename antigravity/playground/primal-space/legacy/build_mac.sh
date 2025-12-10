#!/bin/bash
# Simple macOS build script for CEMQC Core (Clang/Apple Silicon/Intel)
# Usage: ./build_mac.sh [debug|release]

set -e

BUILD=${1:-release}
BUILD_DIR="build-mac"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ "$BUILD" == "debug" ]; then
    cmake -DCMAKE_BUILD_TYPE=Debug -DCEMQC_SELFTEST=ON ..
else
    cmake -DCMAKE_BUILD_TYPE=Release -DCEMQC_SELFTEST=ON ..
fi

cmake --build . -j$(sysctl -n hw.ncpu)

echo
echo "✅ Build complete! Run ./cemqc_selftest_runner"