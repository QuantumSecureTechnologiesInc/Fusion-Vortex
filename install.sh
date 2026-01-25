#!/bin/bash
set -euo pipefail

echo ">>> Bootstrapping Fusion Toolchain..."

DIST="./dist"
rm -rf "$DIST"
mkdir -p "$DIST/bin"
mkdir -p "$DIST/lib/fusion/std"
mkdir -p "$DIST/lib/fusion/core_runtime_evolution"
mkdir -p "$DIST/lib/fusion/forge"
mkdir -p "$DIST/lib/fusion/interop"
mkdir -p "$DIST/haft_nodes"
mkdir -p "$DIST/source_files"

# 1. Build runtime
if [ -f "runtime/runtime.c" ]; then
  echo ">>> Building Runtime..."
  cc -c runtime/runtime.c -o "$DIST/lib/fusion/runtime.o"
else
  echo ">>> Missing runtime/runtime.c"
  exit 1
fi

# 1b. Install ARC runtime source (optional, used by fuc if present)
if [ -f "src/stdlib/arc_runtime.c" ]; then
  echo ">>> Installing ARC runtime..."
  cp "src/stdlib/arc_runtime.c" "$DIST/lib/fusion/arc_runtime.c"
  cc -c "$DIST/lib/fusion/arc_runtime.c" -o "$DIST/lib/fusion/arc_runtime.o" || true
fi

# 2. Build compiler (fuc)
echo ">>> Building Compiler..."
python3 tools/build_fuc_from_fu.py
cp target/release/fuc "$DIST/bin/fuc"

# 3. Build driver (fusion)
echo ">>> Building Build Driver..."
python3 tools/build_fusion_cli_from_fu.py
if [ -f "target/release/fusion" ]; then
  cp target/release/fusion "$DIST/bin/fusion"
else
  echo ">>> fusion binary not found after build"
  exit 1
fi

# 4. Install Standard Library Sources
if [ -d "registry/crates/std" ]; then
  echo ">>> Installing Standard Library..."
  rm -rf "$DIST/lib/fusion/std"
  mkdir -p "$DIST/lib/fusion/std/src"
  cp -f registry/crates/std/src/lib.fu "$DIST/lib/fusion/std/src/"
  cp -f registry/crates/std/src/main.fu "$DIST/lib/fusion/std/src/"
else
  echo ">>> Missing registry/crates/std"
  exit 1
fi

# 5. Build and install Core Runtime Evolution
if [ -d "ecosystem/core_runtime_evolution" ]; then
  echo ">>> Building Core Runtime Evolution..."
  python3 tools/build_core_runtime_evolution_from_fu.py
  cp -r ecosystem/core_runtime_evolution/* "$DIST/lib/fusion/core_runtime_evolution/"
  find "$DIST/lib/fusion/core_runtime_evolution" -name "*.rs" -print -delete
else
  echo ">>> Missing ecosystem/core_runtime_evolution"
  exit 1
fi

# 6. Build and install HAFT Nodes
if [ -d "ecosystem/haft_mesh_nodes_v1" ]; then
  echo ">>> Building HAFT Nodes..."
  python3 tools/build_haft_nodes_from_fu.py
else
  echo ">>> Missing ecosystem/haft_mesh_nodes_v1"
  exit 1
fi

# 7. Install Fusion Forge sources
if [ -d "tools/forge" ]; then
  echo ">>> Installing Fusion Forge..."
  cp -r tools/forge/* "$DIST/lib/fusion/forge/"
  find "$DIST/lib/fusion/forge" -name "*.rs" -print -delete
else
  echo ">>> Missing tools/forge"
  exit 1
fi

# 8. Install Source Files payload
if [ -d "Source Files" ]; then
  echo ">>> Installing Source Files..."
  python3 - <<'PY'
from pathlib import Path
import shutil
root = Path(__file__).resolve().parent
source_root = root / "Source Files"
dist = root / "dist" / "source_files"
dist.mkdir(parents=True, exist_ok=True)
items = [
    "The HAFT Libraries",
    "Fusion Forge (The Build System)",
    "Fusion Entropic Borrow Checker",
    "Fusion Forge",
    "FUSION MCP v1.0 — FORMAL SPECIFIC",
    "Fusion Runtime Core Upgrade",
    "Fusion Runtime Core v2.0 (Nebula)",
    "Fusion Runtime Core v3.0 (Supernova)",
    "Fusion TUI",
    "Fusion Unified  Monolith",
    "Fusion VSC CLi Next-Level Upgrade",
    "Fusion VSC Upgrade",
    "HAFT Engines",
    "HAFT Mesh Nodes v1.0",
    "Hyper-Adaptive Flux Tensor (HAFT)",
    "Intergrations",
    "neuralseal_pqc",
    "ReactorCLI",
    "Sentinel TriBrid",
    "Standard Lib",
    "stdlib",
    "TensorWeave",
    "TermBlink",
    "The Core Runtime Evolution",
]
for item in items:
    src = source_root / item
    if not src.exists():
        print(f">>> Missing Source Files item: {src}")
        continue
    dst = dist / item
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst)
PY
else
  echo ">>> Missing Source Files"
  exit 1
fi

# 9. Install interop assets (JS/TS/Python/etc.)
if [ -d "toolchain/interop" ]; then
  echo ">>> Installing Interop Assets..."
  cp -r toolchain/interop/* "$DIST/lib/fusion/interop/"
else
  echo ">>> Missing toolchain/interop"
  exit 1
fi

echo ">>> Fusion Toolchain Ready in $DIST"
echo ">>> To use, add $PWD/$DIST/bin to your PATH"
