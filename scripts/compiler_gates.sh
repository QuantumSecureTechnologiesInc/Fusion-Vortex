#!/usr/bin/env bash
set -euo pipefail
ROOT="/mnt/c/Projects/Fusion - Programming Language"
FUC="$ROOT/dist/bin/fuc"
FIXT="$ROOT/crates/fuc/tests/fixtures"

# Parse-only and sema-only checks on fixtures
for fu in "$FIXT"/*.fu; do
  "$FUC" "$fu" --parse-only > /dev/null
  "$FUC" "$fu" --sema-only > /dev/null
  echo "ok gate $(basename "$fu")"
done

echo "OK: compiler gates passed"
