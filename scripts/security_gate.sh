#!/usr/bin/env bash
set -euo pipefail
ROOT="/mnt/c/Projects/Fusion - Programming Language"

# 1) Disallow unsafe in user‑facing stdlib
STD="$ROOT/dist/lib/fusion/std"
if [ -d "$STD" ] && rg -n "\bunsafe\b" "$STD" --glob "*.fu" >/dev/null; then
  echo "FAIL: unsafe found in stdlib .fu" >&2
  rg -n "\bunsafe\b" "$STD" --glob "*.fu" >&2
  exit 1
fi

# 2) dist stdlib must be only lib.fu and main.fu
STD_SRC="$ROOT/dist/lib/fusion/std/src"
if [ ! -d "$STD_SRC" ]; then
  echo "FAIL: stdlib not installed in dist" >&2
  exit 1
fi
bad_std=$(find "$STD_SRC" -type f ! -name "lib.fu" ! -name "main.fu" | wc -l | tr -d ' ')
if [ "$bad_std" != "0" ]; then
  echo "FAIL: stdlib contains non‑canonical files" >&2
  find "$STD_SRC" -type f ! -name "lib.fu" ! -name "main.fu" >&2
  exit 1
fi

# 3) dist should not ship .rs or .fsn outside interop assets
if find "$ROOT/dist" -type f \( -name "*.rs" -o -name "*.fsn" \) -not -path "$ROOT/dist/lib/fusion/interop/*" | rg . >/dev/null; then
  echo "FAIL: dist contains .rs/.fsn outside interop" >&2
  find "$ROOT/dist" -type f \( -name "*.rs" -o -name "*.fsn" \) -not -path "$ROOT/dist/lib/fusion/interop/*" >&2
  exit 1
fi

# 4) interop assets isolated
if [ ! -d "$ROOT/toolchain/interop" ]; then
  echo "FAIL: interop assets missing" >&2
  exit 1
fi

echo "OK: security gate passed"
