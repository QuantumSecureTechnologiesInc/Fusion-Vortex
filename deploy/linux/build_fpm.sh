#!/bin/bash
set -euo pipefail

DIST_PATH=${1:-"./dist"}
OUT_DEB=${2:-"fusion-toolchain.deb"}
OUT_RPM=${3:-"fusion-toolchain.rpm"}
VERSION="1.0.0"

if [ ! -d "$DIST_PATH" ]; then
  echo "dist not found: $DIST_PATH" >&2
  exit 1
fi

if ! command -v fpm >/dev/null 2>&1; then
  echo "fpm not found. Install fpm (RubyGem) to build packages." >&2
  exit 1
fi

fpm -s dir -t deb -n fusion-toolchain -v "$VERSION" \
  --prefix /opt/fusion \
  "$DIST_PATH/= /opt/fusion/dist" \
  -p "$OUT_DEB"

fpm -s dir -t rpm -n fusion-toolchain -v "$VERSION" \
  --prefix /opt/fusion \
  "$DIST_PATH/= /opt/fusion/dist" \
  -p "$OUT_RPM"

echo "DEB built: $OUT_DEB"
 echo "RPM built: $OUT_RPM"
