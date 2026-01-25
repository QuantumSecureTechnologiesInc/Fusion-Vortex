# Fusion Terminal Browser v0.2

A powerful terminal-based web browser using **Chromiumoxide** (superior CDP implementation) with the **Blink rendering engine** and **WebGPU** acceleration, powered by **Fusion Runtime Core** instead of Tokio.

## What's New in v0.2

- ✅ **Chromiumoxide Integration**: Superior Chrome DevTools Protocol support
- ✅ **Fusion Runtime Core**: Custom async runtime replacing Tokio
- ✅ **Enhanced WebGPU Support**: Better GPU acceleration with `--enable-unsafe-webgpu`
- ✅ **2x Vertical Resolution**: Half-block rendering for sharper output
- ✅ **Better Mouse Support**: Coordinate-based clicking with CDP
- ✅ **Improved Performance**: Faster rendering and event handling

## Architecture

```text
┌─────────────────────────────────────────┐
│         Browser (Orchestration)         │
├─────────────────────────────────────────┤
│  Fusion Runtime Core (not Tokio!)       │
├──────┬──────────┬──────────┬────────────┤
│Engine│ Renderer │    UI    │   WebGPU   │
│(CDP) │(HalfBlk) │(ratatui) │   (wgpu)   │
├──────┴──────────┴──────────┴────────────┤
│        chromiumoxide (not headless_     │
│        chrome - better CDP!)            │
└─────────────────────────────────────────┘
```text

## Key Improvements

### Chromiumoxide vs headless_chrome

| Feature            | chromiumoxide      | headless_chrome |
| ------------------ | ------------------ | --------------- |
| CDP Support        | ✅ Full async       | ⚠️ Limited       |
| WebGPU             | ✅ Native           | ❌ Experimental  |
| Performance        | ✅ Faster           | ⚠️ Slower        |
| Mouse Events       | ✅ Coordinate-based | ⚠️ Selector-only |
| Active Development | ✅ Yes              | ⚠️ Stale         |

### Fusion Runtime Core vs Tokio

| Feature           | Fusion Runtime         | Tokio             |
| ----------------- | ---------------------- | ----------------- |
| Hybrid Workloads  | ✅ Quantum/AI/Classical | ❌ General purpose |
| Custom Scheduler  | ✅ Fiber-based          | ⚠️ Work-stealing   |
| GPU Integration   | ✅ Native HAL           | ❌ None            |
| Memory Management | ✅ Custom allocators    | ⚠️ System          |

## Features

- **Full Web Rendering**: Complete HTML, CSS, and JavaScript support via Chromium
- **Multiple Render Modes**: ASCII, Unicode blocks, True color with half-blocks
- **WebGPU Acceleration**: GPU-accelerated image processing and WebGPU content support
- **Interactive UI**: Full-featured terminal interface with keyboard/mouse support
- **Session Management**: Persistent history, bookmarks, and cookies
- **JavaScript Execution**: Execute arbitrary JavaScript and capture results
- **Screenshot Capture**: Save screenshots as PNG files
- **Headless Automation**: Perfect for testing and scraping
- **CLI Integration**: Designed for command-line workflows

## Installation

### Prerequisites

1. **Chrome/Chromium** (auto-detected or set `CHROME_PATH`)
2. **Rust 1.70+**
3. **Fusion Runtime Core** (included in workspace)

### Build

```bash
cd crates/fusion-terminal-browser
cargo build --release
```text

## Usage

### Interactive Browser

```bash

# Start with default URL

fusion-browser

# Navigate to specific URL

fusion-browser --url https://webgpu.github.io/webgpu-samples/

# Custom viewport and FPS

fusion-browser --url https://example.com --width 1920 --height 1080 --fps 30
```text

### Screenshot Capture

```bash
fusion-browser screenshot https://example.com --output screenshot.png
```text

### Execute JavaScript

```bash
fusion-browser exec https://example.com --script "document.title"
```text

### Configuration

```bash

# Disable WebGPU

fusion-browser --url https://example.com --no-webgpu

# ASCII mode for compatibility

fusion-browser --url https://example.com --render-mode ascii

# Custom quality

fusion-browser --url https://example.com --quality 80
```text

## Library Usage

```rust
use fusion_terminal_browser::{Browser, BrowserConfig};

fn main() ->Result<(), Box<dyn std::error::Error>> {
    // Configuration
    let config = BrowserConfig::default();

    // Create browser (uses Fusion Runtime Core internally)
    let mut browser = Browser::new(config)?;

    // Navigate
    browser.navigate("https://example.com")?;

    // Execute JavaScript
    let title = browser.execute_script("document.title")?;
    println!("Title: {}", title);

    // Screenshot
    browser.screenshot_to_file("screenshot.png".as_ref())?;

    Ok(())
}
```text

## Keyboard Shortcuts

- **Ctrl+C/Esc**: Quit
- **Ctrl+R**: Reload
- **←/→**: Navigate history
- **↑/↓**: Scroll page
- **Ctrl+G**: Navigate to URL

## Mouse Support

- **Click**: Interactive element activation
- **Scroll**: Page navigation
- **Drag**: Selection (where supported)

## Advanced Features

### WebGPU Content

The browser natively supports WebGPU content:

```bash
fusion-browser https://webgpu.github.io/webgpu-samples/ --fps 30
```text

### Custom Chrome Args

```rust
let mut config = BrowserConfig::default();
config.chrome_args.push("--use-angle=vulkan".to_string());
config.chrome_args.push("--force-device-scale-factor=2".to_string());
```text

### Fusion Runtime Integration

Access the runtime for advanced control:

```rust
let runtime = Arc::new(Runtime::builder()
    .enable_gpu()
    .enable_qpu()  // Quantum processing
    .worker_threads(8)
    .build());
```text

## Performance

Typical performance (1920x1080 → 120x40 terminal):

- **Page Load**: 1-2 seconds
- **Screenshot**: 200-500ms
- **Processing**: 50-150ms (with WebGPU)
- **Rendering**: 20-50ms (half-blocks)
- **Frame Rate**: Up to 60 FPS

## Dependencies

### Core

- `chromiumoxide`: Chrome DevTools Protocol
- `fusion_runtime_core`: Custom async runtime
- `wgpu`: WebGPU API
- `crossterm`: Terminal manipulation
- `ratatui`: Terminal UI
- `image`: Image processing

### Why Not Tokio?

Fusion Runtime Core provides:
1. **Better GPU integration** through HAL
2. **Quantum workload support** via QPU sequencer
3. **Hybrid scheduling** for AI/Classical/Quantum tasks
4. **Custom memory management** for performance
5. **Native Fusion ecosystem** integration

## Troubleshooting

### Chrome Not Found

Set the `CHROME_PATH` environment variable or run setup scripts:

**Windows:**

```powershell

# Download Chrome for Testing

irm https://googlechromelabs.github.io/chrome-for-testing/latest-win64.json | ConvertFrom-Json | %{ $_.channels.Stable.downloads.chrome[0].url } | %{ iwr $_ -OutFile chrome.zip }
Expand-Archive chrome.zip ./bin/
```text

**Linux:**

```bash

# Download Chrome for Testing

wget -O chrome.zip https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/latest/linux64/chrome-linux64.zip
unzip chrome.zip -d ./bin/
```text

### WebGPU Errors

WebGPU requires specific Chrome flags. The browser automatically adds:
- `--enable-unsafe-webgpu`
- `--disable-gpu-sandbox`
- `--use-angle=vulkan` (Linux)

### Performance Issues

1. **Reduce FPS**: `--fps 10`
2. **Lower quality**: `--quality 30`
3. **Disable WebGPU**: `--no-webgpu`
4. **Smaller viewport**: `--width 1280 --height 720`

## Roadmap

### v0.3.0

- [ ] Multiple tabs
- [ ] Better input handling
- [ ] Form auto-fill
- [ ] Cookie encryption
- [ ] Network throttling

### v1.0.0

- [ ] Stable API
- [ ] Production-ready
- [ ] Full automation suite
- [ ] Performance profiling
- [ ] CI/CD integration

## Comparison with TermBlink

This is the productionized version of TermBlink with:
- ✅ Library API (not just binary)
- ✅ Session management
- ✅ Multiple render modes
- ✅ Configuration system
- ✅ Comprehensive documentation
- ✅ Fusion ecosystem integration
- ✅ Custom runtime (not Tokio)

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md)

## Licence

MIT / Apache-2.0

## Credits

- Based on TermBlink architecture
- Powered by chromiumoxide
- Uses Fusion Runtime Core
- Inspired by browsh and carbonyl

---

**Fusion Terminal Browser** - Bringing the web to your terminal with unprecedented power and flexibility.