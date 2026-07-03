#!/usr/bin/env bash
set -euo pipefail

# Updated 2026-06-25: uses relative paths from script location.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACTS="$ROOT/artifacts/native-bootstrap"
mkdir -p "$ARTIFACTS"

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "missing required command: $1" >&2
    exit 1
  }
}

need_cmd python3
need_cmd llc
need_cmd clang

echo ">>> Fusion native bootstrap (Linux/macOS)"
STAGE0=""
if [[ -x "$ROOT/bin/fuc" ]]; then
  STAGE0="$ROOT/bin/fuc"
elif [[ -f "$ROOT/bin/fuc.exe" ]]; then
  STAGE0="$ROOT/bin/fuc.exe"
elif [[ -x "$ROOT/target/release/fuc" ]]; then
  STAGE0="$ROOT/target/release/fuc"
elif [[ -x "$ROOT/target/release/fuc.exe" ]]; then
  STAGE0="$ROOT/target/release/fuc.exe"
fi
if [[ -z "$STAGE0" ]]; then
  echo "missing stage0 compiler (target/release/fuc or target/release/fuc.exe)" >&2
  exit 1
fi

echo ">>> Reusing stage0 compiler: $STAGE0"
export FUC_STAGE0="$STAGE0"
unset FUC_ALLOW_CARGO
python3 "$ROOT/tools/build_fuc_from_fu.py"

mkdir -p "$ROOT/bin"
if [[ "$STAGE0" == *.exe ]]; then
  cp -f "$STAGE0" "$ROOT/bin/fuc.exe"
else
  cp -f "$STAGE0" "$ROOT/bin/fuc"
fi

cat > "$ARTIFACTS/smoke_main.fu" <<'EOF'
pub fn main() -> int {
    return 0;
}
EOF

if [[ "$STAGE0" == *.exe ]]; then
  "$STAGE0" "$ARTIFACTS/smoke_main.fu" -o "$ARTIFACTS/smoke_main.exe" --emit-bin
else
  "$STAGE0" "$ARTIFACTS/smoke_main.fu" -o "$ARTIFACTS/smoke_main" --emit-bin
  "$ARTIFACTS/smoke_main"
fi

echo ">>> Native bootstrap complete"
