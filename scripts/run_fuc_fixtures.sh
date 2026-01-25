#!/usr/bin/env bash
set -euo pipefail
ROOT="/mnt/c/Projects/Fusion - Programming Language"
FUC="$ROOT/dist/bin/fuc"
RUNTIME="$ROOT/dist/lib/fusion/runtime.o"
FIXT="$ROOT/crates/fuc/tests/fixtures"
TMPDIR=$(mktemp -d)
status=0
for fu in "$FIXT"/*.fu; do
  base=$(basename "$fu" .fu)
  out_expected="$FIXT/$base.out"
  exit_expected="$FIXT/$base.exit"
  obj="$TMPDIR/$base.o"
  exe="$TMPDIR/$base"
  "$FUC" "$fu" -o "$obj" > /dev/null
  cc "$obj" "$RUNTIME" -o "$exe" -no-pie -lc > /dev/null
  if [ -f "$exit_expected" ]; then
    "$exe" > "$TMPDIR/$base.stdout" 2>/dev/null || true
    code=$(cat "$exit_expected")
    if ! [[ "$code" =~ ^[0-9]+$ ]]; then
      echo "FAIL $base: invalid exit code marker" >&2
      status=1
    fi
  else
    output=$($exe)
    if [ -f "$out_expected" ]; then
      expected=$(cat "$out_expected")
      if [ "$output" != "$expected" ]; then
        echo "FAIL $base: output mismatch" >&2
        status=1
      fi
    fi
  fi
  echo "ok $base"
done
exit $status
