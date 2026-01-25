# Fusion Visual Compiler - Desktop Edition

**Native Windows Desktop Application**

A standalone desktop application built with Tauri, combining the power of:
- 🚀 **Supernova Runtime v3.0** - Quantum-native execution engine
- 🔨 **Fusion Forge** - Advanced build system
- ⚡ **ReactorCLI** - Interactive interface
- 🎨 **Premium UI** - Native Windows experience

## Features

✅ **Native Performance** - Rust-powered backend, no Electron overhead
✅ **Offline Capable** - Works without internet connection
✅ **Windows Integration** - Native file dialogs, system tray, notifications
✅ **MSI Installer** - Professional Windows installer package
✅ **Auto-Updates** - Built-in update mechanism (optional)
✅ **Small Footprint** - ~15MB installer vs 100MB+ Electron apps

## Building

### Prerequisites

1. **Rust** (already installed)
2. **Node.js** (for frontend build)
3. **WiX Toolset v3** (for MSI installer)

```powershell

# Install WiX Toolset

winget install WiX.Toolset
```text

### Development Build

```bash
cd cmd/fusion-visual-desktop
cargo tauri dev
```text

This will:
1. Build the Rust backend
2. Start the Next.js frontend in dev mode
3. Launch the desktop app

### Production Build

```bash
cargo tauri build
```text

This creates:
- `target/release/fusion-visual-desktop.exe` - Standalone executable
- `target/release/bundle/msi/Fusion Visual Compiler_1.0.0_x64_en-US.msi` - MSI installer
- `target/release/bundle/nsis/Fusion Visual Compiler_1.0.0_x64-setup.exe` - NSIS installer

## Installation

### For End Users

1. Download `Fusion Visual Compiler_1.0.0_x64_en-US.msi`
2. Double-click to install
3. Launch from Start Menu

### For Developers

```bash

# Install from source

cargo tauri build
cd target/release/bundle/msi
msiexec /i "Fusion Visual Compiler_1.0.0_x64_en-US.msi"
```text

## Architecture

```text
┌─────────────────────────────────────┐
│   Tauri Window (Native WebView)     │
│   ┌─────────────────────────────┐   │
│   │  Next.js Frontend           │   │
│   │  (fusion-visual-ui)         │   │
│   └──────────┬──────────────────┘   │
│              │ IPC                  │
│   ┌──────────▼──────────────────┐   │
│   │  Rust Backend               │   │
│   │  - Supernova Runtime        │   │
│   │  - Intent Engine            │   │
│   │  - Flux Resolver            │   │
│   │  - Code Generator           │   │
│   └─────────────────────────────┘   │
└─────────────────────────────────────┘
```text

## Tauri Commands

The desktop app exposes these commands to the frontend:

```typescript
// Process an intent
await invoke('process_intent', { intent: 'Create ML pipeline' })

// Get build status
await invoke('get_build_status', { sessionId: '...' })

// Open output folder in Explorer
await invoke('open_output_folder', { path: './fusion_build_123' })
```text

## Distribution

### MSI Installer Features

- ✅ Add to PATH automatically
- ✅ Desktop shortcut
- ✅ Start Menu entry
- ✅ Uninstaller
- ✅ Upgrade support

### Code Signing (Optional)

```bash

# Sign the MSI

signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com "Fusion Visual Compiler.msi"
```text

## Troubleshooting

### "VCRUNTIME140.dll not found"

Install Visual C++ Redistributable:

```powershell
winget install Microsoft.VCRedist.2015+.x64
```text

### Build fails with "WiX not found"

Ensure WiX is in PATH:

```powershell
$env:PATH += ";C:\Program Files (x86)\WiX Toolset v3.11\bin"
```text

## License

MIT OR Apache-2.0