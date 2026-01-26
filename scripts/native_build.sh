#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST="$ROOT/dist"

mkdir -p "$DIST/bin" "$DIST/lib/fusion/std" "$DIST/lib/fusion/core_runtime_evolution" \
  "$DIST/lib/fusion/forge" "$DIST/lib/fusion/interop" "$DIST/haft_nodes" "$DIST/source_files"

# 1. Build runtime
if [ -f "$ROOT/runtime/runtime.c" ]; then
  echo ">>> Building Runtime..."
  cc -c "$ROOT/runtime/runtime.c" -o "$DIST/lib/fusion/runtime.o"
else
  echo ">>> Missing runtime/runtime.c" >&2
  exit 1
fi

# 1b. Install ARC runtime source (optional)
if [ -f "$ROOT/src/stdlib/arc_runtime.c" ]; then
  echo ">>> Installing ARC runtime..."
  cp "$ROOT/src/stdlib/arc_runtime.c" "$DIST/lib/fusion/arc_runtime.c"
  cc -c "$DIST/lib/fusion/arc_runtime.c" -o "$DIST/lib/fusion/arc_runtime.o" || true
fi

# 2. Find bootstrap compiler
BOOTSTRAP_FUC="${FUC_BOOTSTRAP:-}"
if [ -z "$BOOTSTRAP_FUC" ] && [ -x "$DIST/bin/fuc" ]; then
  BOOTSTRAP_FUC="$DIST/bin/fuc"
fi
if [ -z "$BOOTSTRAP_FUC" ]; then
  if command -v fuc >/dev/null 2>&1; then
    BOOTSTRAP_FUC="$(command -v fuc)"
  fi
fi
if [ -z "$BOOTSTRAP_FUC" ]; then
  echo ">>> Missing bootstrap compiler. Set FUC_BOOTSTRAP or install fuc on PATH." >&2
  exit 1
fi

# 3. Build compiler (fuc) using .fu + Fusion.toml
export FUSION_STD_PATH="${FUSION_STD_PATH:-$ROOT/registry/crates/std}"
export FUSION_SYSROOT="$DIST"
mkdir -p "$ROOT/target/release"

echo ">>> Building Compiler (native)..."
"$BOOTSTRAP_FUC" "$ROOT/crates/fuc/src/main.fu" --emit-bin -o "$ROOT/target/release/fuc"
cp "$ROOT/target/release/fuc" "$DIST/bin/fuc"

# 4. Install Standard Library Sources
if [ -d "$ROOT/registry/crates/std" ]; then
  echo ">>> Installing Standard Library..."
  rm -rf "$DIST/lib/fusion/std"
  mkdir -p "$DIST/lib/fusion/std/src"
  cp -f "$ROOT/registry/crates/std/src/lib.fu" "$DIST/lib/fusion/std/src/"
  cp -f "$ROOT/registry/crates/std/src/main.fu" "$DIST/lib/fusion/std/src/"
else
  echo ">>> Missing registry/crates/std" >&2
  exit 1
fi

# 5. Install Source Files payload (optional)
if [ -d "$ROOT/source_archives/Source Files" ]; then
  echo ">>> Installing Source Files..."
  python3 - <<'PY'
from pathlib import Path
import shutil
root = Path(__file__).resolve().parent.parent
source_root = root / "source_archives" / "Source Files"
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
fi

# 6. Optional payload installs (no native build required)
if [ -d "$ROOT/tools/forge" ]; then
  echo ">>> Installing Fusion Forge..."
  cp -r "$ROOT/tools/forge"/* "$DIST/lib/fusion/forge/"
  find "$DIST/lib/fusion/forge" -name "*.rs" -print -delete
fi

if [ -d "$ROOT/toolchain/interop" ]; then
  echo ">>> Installing Interop Assets..."
  cp -r "$ROOT/toolchain/interop"/* "$DIST/lib/fusion/interop/"
fi

echo ">>> Skipped cargo-based builds (core_runtime_evolution, haft_nodes, fusion CLI)."
echo ">>> Native build complete in $DIST"
