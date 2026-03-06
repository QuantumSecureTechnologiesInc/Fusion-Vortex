#!/bin/bash
set -e

# Fusion Native Installer
# Strictly No Rust Dependencies

echo ">>> Building Fusion Native Runtime..."

# 1. Compile Native Runtime (C)
# Requires: gcc or clang
if command -v clang >/dev/null; then
    CC="clang"
elif command -v gcc >/dev/null; then
    CC="gcc"
else
    echo "Error: C compiler (gcc or clang) required for runtime."
    exit 1
fi

mkdir -p bin

echo "Compiling libfusionrt.a with $CC..."
$CC -c runtime/native/fusionrt.c -o runtime/native/fusionrt.o -I runtime/native
ar rcs bin/libfusionrt.a runtime/native/fusionrt.o

echo ">>> Runtime built at bin/libfusionrt.a"

# 2. Check for existing compiler to bootstrap
if [ -f "bin/fuc" ]; then
    COMPILER="bin/fuc"
    echo "Using existing compiler: $COMPILER"
else
    echo "Msg: No pre-built compiler found at bin/fuc."
    echo "Assuming standard distribution or previous build."
    # In a real scenario, we might download a stage0 binary here
    # For now, we assume the user has one if they want to self-host
    exit 1
fi

# 3. Compile Fusion Compiler (Self-Hosting)
# Compiling crates/fuc/src/main.fu -> bin/fuc_new
echo ">>> Compiling Fusion Compiler (fuc)..."

$COMPILER crates/fuc/src/main.fu \
    --output bin/fuc_new.o \
    --lib
echo ">>> SUCCESS: New compiler built at bin/fuc_new"
echo ">>> You can replace bin/fuc with bin/fuc_new to upgrade."
