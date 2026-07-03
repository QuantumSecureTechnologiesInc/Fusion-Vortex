#!/bin/bash
set -e

# THE OUROBOROS PROTOCOL: Self-Genesis of the Fusion Toolchain
echo ">>> Initialising Ouroboros Phase <<<"

# Stage 0: Rust Seed
echo "[1/3] Stage 0: Rust Compiler -> Fusion Compiler"
cargo build --release -p fuc
./target/release/fuc self_hosted/main.fu -o bin/fuc_s0

# Stage 1: First Breath
echo "[2/3] Stage 1: Fusion Compiler -> Fusion Compiler"
./bin/fuc_s0 self_hosted/main.fu -o bin/fuc_s1

# Stage 2: Finality
echo "[3/3] Stage 2: Verification"
./bin/fuc_s1 self_hosted/main.fu -o bin/fuc_s2

if diff bin/fuc_s1 bin/fuc_s2; then
    echo ">>> Sovereignty Verified: Fusion is now independent. <<<"
    cp bin/fuc_s2 dist/bin/fusion
else
    echo ">>> Critical Error: Bootstrap divergence detected. <<<"
    exit 1
fi