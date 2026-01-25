# Fusion Visual Compiler - Complete Suite

## Three Deployment Options

### 1. **Web Version** (`fusion-visual`)

- Rust backend + Next.js frontend
- Access via browser at `http://localhost:3000`
- Best for: Quick prototyping, remote access

```bash
cd cmd/fusion-visual
cargo run
```text

### 2. **Native Backend** (`fusion-visual-native`)

- Powered by **Supernova Runtime v3.0**
- Integrated **Fusion Forge** + **ReactorCLI**
- Same web UI, enhanced backend
- Best for: Production use, maximum performance

```bash
cd cmd/fusion-visual-native
cargo run --release
```text

### 3. **Desktop App** (`fusion-visual-desktop`) ⭐ **RECOMMENDED**

- **Native Windows application** with Tauri
- **MSI Installer** for easy distribution
- **Offline capable**, no browser required
- **15MB installer** (vs 100MB+ Electron)
- Best for: End users, offline work, professional deployment

```bash
cd cmd/fusion-visual-desktop

# Development

cargo tauri dev

# Production build (creates MSI)

cargo tauri build
```text

## Quick Comparison

| Feature            | Web  | Native | Desktop |
| ------------------ | ---- | ------ | ------- |
| Browser Required   | ✅    | ✅      | ❌       |
| Supernova Runtime  | ❌    | ✅      | ✅       |
| Offline Mode       | ❌    | ❌      | ✅       |
| MSI Installer      | ❌    | ❌      | ✅       |
| File Size          | ~5MB | ~10MB  | ~15MB   |
| Startup Time       | Fast | Fast   | Instant |
| System Integration | None | None   | Full    |

## Installation (Desktop)

### For End Users

1. Download `Fusion Visual Compiler_1.0.0_x64_en-US.msi`
2. Double-click to install
3. Launch from Start Menu

### For Developers

```bash

# Build installer

cd cmd/fusion-visual-desktop
cargo tauri build

# Install

cd target/release/bundle/msi
msiexec /i "Fusion Visual Compiler_1.0.0_x64_en-US.msi"
```text

## Features

All versions include:
- 🎨 **Premium UI** - Glassmorphism design
- 🧠 **AI Intent Parsing** - Natural language to code
- 🔧 **Flux Resolver** - Smart dependency management
- 📦 **Code Generation** - ML, Web, Quantum templates
- ⚡ **Real-time Feedback** - Live build visualization

Desktop version adds:
- 🪟 **Native Windows** - System tray, notifications
- 📁 **File Integration** - Native dialogs, Explorer integration
- 🔒 **Offline First** - No internet required
- 🚀 **Auto-updates** - Built-in update mechanism

## Architecture

```text
┌─────────────────────────────────────────────┐
│           Fusion Visual Compiler            │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────────┐  ┌──────────────────────┐ │
│  │   Web UI    │  │  Desktop (Tauri)     │ │
│  │  (Browser)  │  │  (Native Window)     │ │
│  └──────┬──────┘  └──────────┬───────────┘ │
│         │                    │             │
│         └────────┬───────────┘             │
│                  │                         │
│         ┌────────▼─────────┐               │
│         │  Backend Engine  │               │
│         ├──────────────────┤               │
│         │ Supernova v3.0   │               │
│         │ Fusion Forge     │               │
│         │ ReactorCLI       │               │
│         └──────────────────┘               │
│                                             │
└─────────────────────────────────────────────┘
```text

## License

MIT OR Apache-2.0

---

**QuantumSecure Technologies Ltd** © 2026