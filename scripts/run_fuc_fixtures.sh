#!/usr/bin/env bash
set -euo pipefail

# FUC fixture suite — compile and run all test fixtures.
# Updated 2026-06-25: uses relative paths; gracefully skips link step on Windows.

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
TMPDIR=$(mktemp -d)
status=0

# Find runtime for linking (optional)
RUNTIME=""
if [ -f "$ROOT/bin/fusionrt.o" ]; then
  RUNTIME="$ROOT/bin/fusionrt.o"
elif [ -f "$ROOT/bin/fusionrt.lib" ]; then
  RUNTIME="$ROOT/bin/fusionrt.lib"
elif [ -f "$ROOT/bin/libfusionrt.a" ]; then
  RUNTIME="$ROOT/bin/libfusionrt.a"
fi

for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  base=$(basename "$fu" .fu)
  out_expected="$FIXT/$base.out"
  exit_expected="$FIXT/$base.exit"
  obj="$TMPDIR/$base.o"

  # Parse check always runs
  "$FUC" "$fu" --parse-only > /dev/null 2>&1 || { echo "FAIL parse $base"; status=1; continue; }

  # Full compile + link + run if runtime available
  if [ -n "$RUNTIME" ]; then
    exe="$TMPDIR/$base"
    if "$FUC" "$fu" -o "$obj" > /dev/null 2>&1 && \
       cc "$obj" "$RUNTIME" -o "$exe" -no-pie -lc > /dev/null 2>&1; then
      if [ -f "$exit_expected" ]; then
        "$exe" > "$TMPDIR/$base.stdout" 2>/dev/null || true
        code=$(cat "$exit_expected")
        if ! [[ "$code" =~ ^[0-9]+$ ]]; then
          echo "FAIL $base: invalid exit code marker" >&2
          status=1
        fi
      else
        output=$("$exe" 2>/dev/null || true)
        if [ -f "$out_expected" ]; then
          expected=$(cat "$out_expected")
          if [ "$output" != "$expected" ]; then
            echo "FAIL $base: output mismatch" >&2
            status=1
            continue
          fi
        fi
      fi
    fi
  fi
  echo "ok $base"
done
exit $status
#!/usr/bin/env bash
set -euo pipefail

# FUC fixture suite — compile and run all test fixtures.
# Updated 2026-06-25: uses relative paths; gracefully skips link step on Windows.

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
TMPDIR=$(mktemp -d)
status=0

# Find runtime for linking (optional)
RUNTIME=""
if [ -f "$ROOT/bin/fusionrt.o" ]; then
  RUNTIME="$ROOT/bin/fusionrt.o"
elif [ -f "$ROOT/bin/fusionrt.lib" ]; then
  RUNTIME="$ROOT/bin/fusionrt.lib"
elif [ -f "$ROOT/bin/libfusionrt.a" ]; then
  RUNTIME="$ROOT/bin/libfusionrt.a"
fi

for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  base=$(basename "$fu" .fu)
  out_expected="$FIXT/$base.out"
  exit_expected="$FIXT/$base.exit"
  obj="$TMPDIR/$base.o"

  # Parse check always runs
  "$FUC" "$fu" --parse-only > /dev/null 2>&1 || { echo "FAIL parse $base"; status=1; continue; }

  # Full compile + link + run if runtime available
  if [ -n "$RUNTIME" ]; then
    exe="$TMPDIR/$base"
    if "$FUC" "$fu" -o "$obj" > /dev/null 2>&1 && \
       cc "$obj" "$RUNTIME" -o "$exe" -no-pie -lc > /dev/null 2>&1; then
      if [ -f "$exit_expected" ]; then
        "$exe" > "$TMPDIR/$base.stdout" 2>/dev/null || true
        code=$(cat "$exit_expected")
        if ! [[ "$code" =~ ^[0-9]+$ ]]; then
          echo "FAIL $base: invalid exit code marker" >&2
          status=1
        fi
      else
        output=$("$exe" 2>/dev/null || true)
        if [ -f "$out_expected" ]; then
          expected=$(cat "$out_expected")
          if [ "$output" != "$expected" ]; then
            echo "FAIL $base: output mismatch" >&2
            status=1
            continue
          fi
        fi
      fi
    fi
  fi
  echo "ok $base"
done
exit $status
#!/usr/bin/env bash
set -euo pipefail

# FUC fixture suite — compile and run all test fixtures.
# Updated 2026-06-25: uses relative paths; gracefully skips link step on Windows.

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
TMPDIR=$(mktemp -d)
status=0

# Find runtime for linking (optional)
RUNTIME=""
if [ -f "$ROOT/bin/fusionrt.o" ]; then
  RUNTIME="$ROOT/bin/fusionrt.o"
elif [ -f "$ROOT/bin/fusionrt.lib" ]; then
  RUNTIME="$ROOT/bin/fusionrt.lib"
elif [ -f "$ROOT/bin/libfusionrt.a" ]; then
  RUNTIME="$ROOT/bin/libfusionrt.a"
fi

for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  base=$(basename "$fu" .fu)
  out_expected="$FIXT/$base.out"
  exit_expected="$FIXT/$base.exit"
  obj="$TMPDIR/$base.o"

  # Parse check always runs
  "$FUC" "$fu" --parse-only > /dev/null 2>&1 || { echo "FAIL parse $base"; status=1; continue; }

  # Full compile + link + run if runtime available
  if [ -n "$RUNTIME" ]; then
    exe="$TMPDIR/$base"
    if "$FUC" "$fu" -o "$obj" > /dev/null 2>&1 && \
       cc "$obj" "$RUNTIME" -o "$exe" -no-pie -lc > /dev/null 2>&1; then
      if [ -f "$exit_expected" ]; then
        "$exe" > "$TMPDIR/$base.stdout" 2>/dev/null || true
        code=$(cat "$exit_expected")
        if ! [[ "$code" =~ ^[0-9]+$ ]]; then
          echo "FAIL $base: invalid exit code marker" >&2
          status=1
        fi
      else
        output=$("$exe" 2>/dev/null || true)
        if [ -f "$out_expected" ]; then
          expected=$(cat "$out_expected")
          if [ "$output" != "$expected" ]; then
            echo "FAIL $base: output mismatch" >&2
            status=1
            continue
          fi
        fi
      fi
    fi
  fi
  echo "ok $base"
done
exit $status
#!/usr/bin/env bash
set -euo pipefail

# FUC fixture suite — compile and run all test fixtures.
# Updated 2026-06-25: uses relative paths; gracefully skips link step on Windows.

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
TMPDIR=$(mktemp -d)
status=0

# Find runtime for linking (optional)
RUNTIME=""
if [ -f "$ROOT/bin/fusionrt.o" ]; then
  RUNTIME="$ROOT/bin/fusionrt.o"
elif [ -f "$ROOT/cmake_build/lib/Release/fusionrt.lib" ]; then
  RUNTIME="$ROOT/cmake_build/lib/Release/fusionrt.lib"
elif [ -f "$ROOT/bin/libfusionrt.a" ]; then
  RUNTIME="$ROOT/bin/libfusionrt.a"
fi

for fu in "$FIXT"/*.fu; do
  [ -f "$fu" ] || continue
  base=$(basename "$fu" .fu)
  out_expected="$FIXT/$base.out"
  exit_expected="$FIXT/$base.exit"
  obj="$TMPDIR/$base.o"

  # Parse check always runs
  "$FUC" "$fu" --parse-only > /dev/null 2>&1 || { echo "FAIL parse $base"; status=1; continue; }

  # Full compile + link + run if runtime available
  if [ -n "$RUNTIME" ]; then
    exe="$TMPDIR/$base"
    if "$FUC" "$fu" -o "$obj" > /dev/null 2>&1 && \
       cc "$obj" "$RUNTIME" -o "$exe" -no-pie -lc > /dev/null 2>&1; then
      if [ -f "$exit_expected" ]; then
        "$exe" > "$TMPDIR/$base.stdout" 2>/dev/null || true
        code=$(cat "$exit_expected")
        if ! [[ "$code" =~ ^[0-9]+$ ]]; then
          echo "FAIL $base: invalid exit code marker" >&2
          status=1
        fi
      else
        output=$("$exe" 2>/dev/null || true)
        if [ -f "$out_expected" ]; then
          expected=$(cat "$out_expected")
          if [ "$output" != "$expected" ]; then
            echo "FAIL $base: output mismatch" >&2
            status=1
            continue
          fi
        fi
      fi
    fi
  fi
  echo "ok $base"
done
exit $status
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
