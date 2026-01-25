#!/bin/bash
set -euo pipefail

DIST_PATH=${1:-"./dist"}
OUT_PKG=${2:-"fusion-toolchain.pkg"}
IDENTIFIER="com.quantumsecure.fusion.toolchain"
VERSION="1.0.0"

if [ ! -d "$DIST_PATH" ]; then
  echo "dist not found: $DIST_PATH" >&2
  exit 1
fi

STAGE_DIR="/tmp/fusion_pkg_root"
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR/usr/local/fusion"
cp -R "$DIST_PATH" "$STAGE_DIR/usr/local/fusion/dist"

pkgbuild \
  --root "$STAGE_DIR" \
  --identifier "$IDENTIFIER" \
  --version "$VERSION" \
  --install-location "/" \
  "$OUT_PKG"

echo "PKG built: $OUT_PKG"
