#!/bin/bash
set -euo pipefail

echo ">>> Bootstrapping Fusion Toolchain (native-only)..."

if [ ! -x "scripts/native_build.sh" ]; then
  echo ">>> Missing scripts/native_build.sh" >&2
  exit 1
fi

scripts/native_build.sh
