#!/usr/bin/env bash
set -euo pipefail

# Security gate — validates stdlib safety and interop isolation.
# Updated 2026-06-25: uses relative paths; checks stdlib/ directly (dist/ not yet built).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# 1) Disallow unsafe in user-facing stdlib
STD="$ROOT/stdlib"
if [ -d "$STD" ] && rg -n "\bunsafe\b" "$STD" --glob "*.fu" >/dev/null 2>&1; then
  echo "FAIL: unsafe found in stdlib .fu" >&2
  rg -n "\bunsafe\b" "$STD" --glob "*.fu" >&2
  exit 1
fi

# 2) Stdlib source check — must contain only lib.fu and main.fu at root
STD_SRC="$ROOT/stdlib"
if [ ! -d "$STD_SRC" ]; then
  echo "FAIL: stdlib directory not found" >&2
  exit 1
fi
# Check for unexpected files at the top level of stdlib/ (subdirs are OK)
bad_std=$(find "$STD_SRC" -maxdepth 1 -type f ! -name "*.fu" ! -name "*.md" | wc -l | tr -d ' ')
if [ "$bad_std" != "0" ]; then
  echo "FAIL: stdlib root contains non-canonical files" >&2
  find "$STD_SRC" -maxdepth 1 -type f ! -name "*.fu" ! -name "*.md" >&2
  exit 1
fi

# 3) dist should not ship .rs or .fsn outside interop assets (if dist/ exists)
if [ -d "$ROOT/dist" ]; then
  if find "$ROOT/dist" -type f \( -name "*.rs" -o -name "*.fsn" \) -not -path "$ROOT/dist/lib/fusion/interop/*" | rg . >/dev/null 2>&1; then
    echo "FAIL: dist contains .rs/.fsn outside interop" >&2
    find "$ROOT/dist" -type f \( -name "*.rs" -o -name "*.fsn" \) -not -path "$ROOT/dist/lib/fusion/interop/*" >&2
    exit 1
  fi
fi

# 4) interop assets isolated
if [ ! -d "$ROOT/toolchain/interop" ]; then
  echo "FAIL: interop assets missing" >&2
  exit 1
fi

echo "OK: security gate passed"
