# Fusion Toolchain Deployment

This folder contains platform-specific **full installer** build scripts.

## Windows (MSI)
- Tool: WiX Toolset v3+
- Script: `deploy/windows/build_msi.ps1`
- Input: `dist/` from `./install.sh` (native compiler `fuc` + runtime assets)
- Output: `fusion-toolchain.msi`

## macOS (PKG)
- Tools: `pkgbuild`, `productbuild` (Xcode Command Line Tools)
- Script: `deploy/macos/build_pkg.sh`
- Input: `dist/` from `./install.sh` (native compiler `fuc` + runtime assets)
- Output: `fusion-toolchain.pkg`

## Linux (DEB/RPM)
- Tool: `fpm`
- Script: `deploy/linux/build_fpm.sh`
- Input: `dist/` from `./install.sh` (native compiler `fuc` + runtime assets)
- Output: `fusion-toolchain.deb`, `fusion-toolchain.rpm`

## Usage
1. Build the toolchain: `./install.sh`
2. Run the platform script above to produce installers.

Note: current packaging installs `fuc` (native compiler) plus runtime/stdlib assets.
The `fusion` CLI is not packaged in the native-only build.

If you need DMG packaging for macOS, we can add `dmgbuild` on top of the PKG.

## macOS DMG (optional)
- Tool: `dmgbuild`
- Script: `deploy/macos/build_dmg.sh`
- Input: `fusion-toolchain.pkg`
- Output: `fusion-toolchain.dmg`
