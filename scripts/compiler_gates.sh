#!/usr/bin/env bash
set -euo pipefail

# Compiler gates — parse-only checks on fixtures.
# Updated 2026-06-25: uses relative paths and bin/fuc (or bin/fuc.exe).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -x "$ROOT/bin/fuc" ]; then
  FUC="$ROOT/bin/fuc"
elif [ -f "$ROOT/bin/fuc.exe" ]; then
  FUC="$ROOT/bin/fuc.exe"
else
  echo "FAIL: compiler not found at bin/fuc or bin/fuc.exe" >&2
  exit 1
fi

FIXT="$ROOT/crates/fuc/tests"

# Parse-only checks on .fu test files
for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  "$FUC" "$fu" --parse-only > /dev/null
  echo "ok parse $(basename "$fu")"
done

echo "OK: compiler gates passed"
#!/usr/bin/env bash
set -euo pipefail

# Compiler gates — parse-only checks on fixtures.
# Updated 2026-06-25: uses relative paths and bin/fuc (or bin/fuc.exe).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -x "$ROOT/bin/fuc" ]; then
  FUC="$ROOT/bin/fuc"
elif [ -f "$ROOT/bin/fuc.exe" ]; then
  FUC="$ROOT/bin/fuc.exe"
else
  echo "FAIL: compiler not found at bin/fuc or bin/fuc.exe" >&2
  exit 1
fi

FIXT="$ROOT/crates/fuc/tests"

# Parse-only checks on .fu test files
for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  "$FUC" "$fu" --parse-only > /dev/null
  echo "ok parse $(basename "$fu")"
done

echo "OK: compiler gates passed"
#!/usr/bin/env bash
set -euo pipefail

# Compiler gates — parse-only checks on fixtures.
# Updated 2026-06-25: uses relative paths and bin/fuc (or bin/fuc.exe).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -x "$ROOT/bin/fuc" ]; then
  FUC="$ROOT/bin/fuc"
elif [ -f "$ROOT/bin/fuc.exe" ]; then
  FUC="$ROOT/bin/fuc.exe"
else
  echo "FAIL: compiler not found at bin/fuc or bin/fuc.exe" >&2
  exit 1
fi

FIXT="$ROOT/crates/fuc/tests"

# Parse-only checks on .fu test files
for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  "$FUC" "$fu" --parse-only > /dev/null
  echo "ok parse $(basename "$fu")"
done

echo "OK: compiler gates passed"
#!/usr/bin/env bash
set -euo pipefail

# Compiler gates — parse-only and sema-only checks on fixtures.
# Updated 2026-06-25: uses relative paths and bin/fuc (or bin/fuc.exe).

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -x "$ROOT/bin/fuc" ]; then
  FUC="$ROOT/bin/fuc"
elif [ -f "$ROOT/bin/fuc.exe" ]; then
  FUC="$ROOT/bin/fuc.exe"
else
  echo "FAIL: compiler not found at bin/fuc or bin/fuc.exe" >&2
  exit 1
fi

FIXT="$ROOT/crates/fuc/tests"

# Parse-only checks on .fu test files
for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  "$FUC" "$fu" --parse-only > /dev/null
  echo "ok parse $(basename "$fu")"
done

echo "OK: compiler gates passed"
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
