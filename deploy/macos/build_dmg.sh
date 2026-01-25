#!/bin/bash
set -euo pipefail

PKG_PATH=${1:-"fusion-toolchain.pkg"}
OUT_DMG=${2:-"fusion-toolchain.dmg"}
VOLUME_NAME="Fusion Toolchain"

if [ ! -f "$PKG_PATH" ]; then
  echo "PKG not found: $PKG_PATH" >&2
  exit 1
fi

if ! command -v dmgbuild >/dev/null 2>&1; then
  echo "dmgbuild not found. Install via: pip install dmgbuild" >&2
  exit 1
fi

TMP_DIR=$(mktemp -d)
cp "$PKG_PATH" "$TMP_DIR/"

dmgbuild -s /dev/null -D volume_name="$VOLUME_NAME" "$VOLUME_NAME" "$OUT_DMG" -T "$TMP_DIR"

rm -rf "$TMP_DIR"

echo "DMG built: $OUT_DMG"
