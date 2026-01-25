#!/usr/bin/env bash
set -euo pipefail
ROOT="/mnt/c/Projects/Fusion - Programming Language"
FUC="$ROOT/dist/bin/fuc"
RUNTIME="$ROOT/dist/lib/fusion/runtime.o"
FIXT="$ROOT/crates/fuc/tests/fixtures"
TMPDIR=$(mktemp -d)

# Build safety_and_repeat and ensure it aborts
fu="$FIXT/safety_and_repeat.fu"
obj="$TMPDIR/safety.o"
exe="$TMPDIR/safety"
"$FUC" "$fu" -o "$obj" > /dev/null
cc "$obj" "$RUNTIME" -o "$exe" -no-pie -lc > /dev/null
set +e
"$exe" > "$TMPDIR/safety.stdout" 2>/dev/null
code=$?
set -e
if [ "$code" -eq 0 ]; then
  echo "FAIL: safety_and_repeat did not abort" >&2
  exit 1
fi

echo "OK: runtime gates passed"
