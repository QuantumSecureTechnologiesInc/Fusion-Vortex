#!/bin/bash
set -euo pipefail

ZIP_PATH=${1:-fusion-toolchain.zip}
INSTALL_ROOT=${2:-$HOME/.local/fusion}

if [ ! -f "$ZIP_PATH" ]; then
  echo "Bundle not found: $ZIP_PATH" >&2
  exit 1
fi

mkdir -p "$INSTALL_ROOT"
rm -rf "$INSTALL_ROOT/dist"

unzip -q "$ZIP_PATH" -d "$INSTALL_ROOT"

BIN_PATH="$INSTALL_ROOT/dist/bin"
PROFILE="$HOME/.zshrc"
if [ -n "${BASH_VERSION:-}" ]; then
  PROFILE="$HOME/.bashrc"
fi

if ! grep -q "$BIN_PATH" "$PROFILE" 2>/dev/null; then
  echo "export PATH=\"$BIN_PATH:\$PATH\"" >> "$PROFILE"
fi

echo "Fusion installed to $INSTALL_ROOT"
echo "Open a new terminal and run: fusion --version"
