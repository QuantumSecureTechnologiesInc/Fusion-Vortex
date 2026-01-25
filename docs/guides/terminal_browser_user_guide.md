# Fusion Terminal Browser - User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Basic Usage](#basic-usage)
4. [Interactive Browsing](#interactive-browsing)
5. [Command-Line Operations](#command-line-operations)
6. [Configuration](#configuration)
7. [Session Management](#session-management)
8. [Troubleshooting](#troubleshooting)

## Introduction

The Fusion Terminal Browser is a powerful terminal-based web browser that brings full web rendering capabilities to the command line. Using the Blink rendering engine (the same engine that powers Chromium and Google Chrome) combined with WebGPU acceleration, it provides a unique way to browse the web, capture screenshots, execute JavaScript, and automate web tasks directly from your terminal.

### Key Features

- **Full Web Rendering**: Uses headless Chrome/Chromium for complete HTML/CSS/JavaScript support
- **GPU Acceleration**: Optional WebGPU support for faster image processing
- **Multiple Display Modes**: ASCII, Unicode blocks, and true color rendering
- **Session Persistence**: Automatic saving of browsing history, bookmarks, and cookies
- **Automation Ready**: Perfect for web scraping, testing, and CI/CD workflows
- **Interactive UI**: Rich terminal interface with keyboard shortcuts

### Who Is This For?

- **Developers** who want to browse and test web applications from the terminal
- **DevOps Engineers** automating web-based workflows
- **Security Researchers** conducting web application assessments
- **Data Scientists** scraping web data
- **Anyone** who prefers terminal-based tools

## Getting Started

### Installation

#### Prerequisites

1. **Chrome or Chromium** must be installed on your system
   - The browser will automatically detect Chrome/Chromium
   - Custom paths can be specified in configuration

2. **Rust Toolchain** (if building from source)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```text

#### Building the Browser

```bash
cd crates/fusion-terminal-browser
cargo build --release
```text

The binary will be located at `target/release/fusion-browser` (or `fusion-browser.exe` on Windows).

#### Adding to PATH

**Linux/macOS:**

```bash
sudo cp target/release/fusion-browser /usr/local/bin/
```text

**Windows (PowerShell as Administrator):**

```powershell
Copy-Item target\release\fusion-browser.exe C:\Windows\System32\
```text

### First Run

Test the installation:

```bash
fusion-browser --url https://example.com
```text

This will start an interactive browser session with example.com loaded.

## Basic Usage

### Browsing a Website

The simplest way to use the browser:

```bash
fusion-browser --url https://your-website.com
```text

This opens an interactive session where you can:
- View the rendered page in your terminal
- Navigate using keyboard shortcuts
- Access browser history

### Taking a Screenshot

Capture a webpage as a PNG image:

```bash
fusion-browser screenshot https://example.com --output screenshot.png
```text

### Viewing Page HTML

Extract the HTML source of a page:

```bash
fusion-browser html https://example.com
```text

### Executing JavaScript

Run JavaScript on a page and get the result:

```bash
fusion-browser exec https://example.com --script "document.title"
```text

## Interactive Browsing

### Starting Interactive Mode

```bash
fusion-browser browse
```text

Or with an initial URL:

```bash
fusion-browser browse --url https://example.com
```text

### Keyboard Shortcuts

| Shortcut          | Action                      |
| ----------------- | --------------------------- |
| `Ctrl+G`          | Navigate to a new URL       |
| `Ctrl+R`          | Reload the current page     |
| `←` (Left Arrow)  | Navigate back in history    |
| `→` (Right Arrow) | Navigate forward in history |
| `Ctrl+Q`          | Quit the browser            |
| `Esc`             | Cancel current operation    |

### Navigation

1. **Going to a URL**: Press `Ctrl+G` and enter the URL
2. **Following Links**: Click on links (if your terminal supports mouse)
3. **Back/Forward**: Use arrow keys to navigate history
4. **Refresh**: Press `Ctrl+R` to reload the page

### Visual Modes

The browser supports multiple rendering modes for different terminals and preferences:

#### ASCII Mode

```bash
fusion-browser --url https://example.com --render-mode ascii
```text

Best for:
- Terminals without Unicode support
- Maximum compatibility
- Low bandwidth situations

#### Unicode Block Mode

```bash
fusion-browser --url https://example.com --render-mode unicode-block
```text

Best for:
- Better visual quality than ASCII
- Most modern terminals
- Good balance of quality and compatibility

#### True Color Mode (Default)

```bash
fusion-browser --url https://example.com --render-mode truecolor
```text

Best for:
- Modern terminals with 24-bit color support
- Maximum visual fidelity
- Recommended for daily use

## Command-Line Operations

### Screenshot Command

Basic usage:

```bash
fusion-browser screenshot <URL> --output <FILE>
```text

Examples:

```bash

# Capture a website

fusion-browser screenshot https://github.com --output github.png

# Capture with specific dimensions (via config)

fusion-browser screenshot https://example.com --output screenshot.png --config my-config.toml
```text

### JavaScript Execution

Execute JavaScript and capture the result:

```bash

# Get page title

fusion-browser exec https://example.com --script "document.title"

# Extract data

fusion-browser exec https://news.ycombinator.com --script "Array.from(document.querySelectorAll('.titleline > a')).map(a => a.textContent)"

# Get page metadata

fusion-browser exec https://example.com --script "({title: document.title, url: location.href, links: document.links.length})"
```text

### HTML Retrieval

```bash

# Get HTML and save to file

fusion-browser html https://example.com > page.html

# Extract specific content with grep

fusion-browser html https://example.com | grep "search-term"
```text

## Configuration

### Configuration File

Create a configuration file to customise the browser:

```bash
fusion-browser config --save browser-config.toml
```text

This creates a TOML file with all available options:

```toml
terminal_width = 120
terminal_height = 40
enable_webgpu = true
enable_javascript = true
enable_websocket = true
enable_images = true
image_quality = 7
page_load_timeout = 30000
window_width = 1920
window_height = 1080
headless = true
enable_gpu = false
render_mode = "TrueColor"
color_depth = "TrueColor"

[user_agent]

# Custom user agent (optional)

[cache_dir]

# Custom cache directory (optional)

[session_file]

# Custom session file path (optional)

```text

### Using a Configuration File

```bash
fusion-browser --config browser-config.toml --url https://example.com
```text

### Runtime Options

Override configuration settings at runtime:

```bash

# Disable WebGPU

fusion-browser --url https://example.com --no-webgpu

# Disable JavaScript

fusion-browser --url https://example.com --no-js

# Disable images

fusion-browser --url https://example.com --no-images

# Set render mode

fusion-browser --url https://example.com --render-mode ascii

# Set color depth

fusion-browser --url https://example.com --color-depth ansi256
```text

### Terminal-Specific Recommendations

#### iTerm2 (macOS)

```toml
render_mode = "TrueColor"
color_depth = "TrueColor"
```text

#### Kitty

```toml
render_mode = "Kitty"  # Uses Kitty graphics protocol
color_depth = "TrueColor"
```text

#### Alacritty

```toml
render_mode = "TrueColor"
color_depth = "TrueColor"
```text

#### Windows Terminal

```toml
render_mode = "TrueColor"
color_depth = "TrueColor"
```text

#### Legacy Terminals

```toml
render_mode = "Ascii"
color_depth = "Ansi16"
```text

## Session Management

### Browsing History

The browser automatically maintains a browsing history. Navigate through it using:

- `←` (Left Arrow): Go back
- `→` (Right Arrow): Go forward

History is persisted between sessions.

### Bookmarks

Bookmarks can be managed programmatically or through the session file.

Session file location (default):
- **Linux/macOS**: `~/.local/share/fusion-browser/session.json`
- **Windows**: `%APPDATA%\fusion-browser\session.json`

### Cookies

Cookie management is automatic. Cookies are:
- Stored in the session file
- Persisted between sessions
- Automatically sent with requests
- Expired cookies are cleaned up

### Clearing Session Data

To clear all session data:

```bash
rm ~/.local/share/fusion-browser/session.json  # Linux/macOS
del %APPDATA%\fusion-browser\session.json       # Windows
```text

## Troubleshooting

### Chrome/Chromium Not Found

**Problem**: Error message about Chrome not being found.

**Solution**:
1. Install Chrome or Chromium
2. Or, specify the path in your configuration:

```toml
[chrome_path]
Some = "/path/to/chrome"
```text

### Page Not Loading

**Problem**: Page fails to load or times out.

**Solution**:
1. Increase the timeout:

   ```toml
   page_load_timeout = 60000  # 60 seconds
```text

2. Check your internet connexion
3. Try disabling JavaScript if the site has issues:

   ```bash
   fusion-browser --url https://example.com --no-js
```text

### Poor Rendering Quality

**Problem**: Page looks distorted or colours are wrong.

**Solution**:
1. Ensure your terminal supports true color:

   ```bash
   echo $COLORTERM  # Should output "truecolor" or "24bit"
```text

2. Try different render modes:

   ```bash
   fusion-browser --url https://example.com --render-mode unicode-block
```text

3. Adjust colour depth:

   ```bash
   fusion-browser --url https://example.com --color-depth ansi256
```text

### WebGPU Errors

**Problem**: WebGPU initialization fails.

**Solution**:
Disable WebGPU (it's optional):

```bash
fusion-browser --url https://example.com --no-webgpu
```text

Or in configuration:

```toml
enable_webgpu = false
```text

### Performance Issues

**Problem**: Browser is slow or uses too much memory.

**Solution**:
1. Disable images:

   ```bash
   fusion-browser --url https://example.com --no-images
```text

2. Reduce window size in configuration:

   ```toml
   window_width = 1024
   window_height = 768
```text

3. Disable JavaScript for static sites:

   ```bash
   fusion-browser --url https://example.com --no-js
```text

### Getting Help

For additional support:

1. Check the [GitHub Issues](https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language/issues)
2. Review the [Technical Documentation](./developer_guide.md)
3. Enable verbose logging:

   ```bash
   fusion-browser --url https://example.com --verbose
```text

## Advanced Usage

### Automation Scripting

Use the browser in shell scripts:

```bash
#!/bin/bash

# Batch screenshot multiple sites

urls=("https://example.com" "https://github.com" "https://rust-lang.org")

for url in "${urls[@]}"; do
    filename=$(echo $url | sed 's|https://||' | sed 's|/|_|g').png
    fusion-browser screenshot "$url" --output "$filename"
    echo "Captured $url to $filename"
done
```text

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Visual Regression Testing
on: [push]
jobs:
  screenshot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Chrome
        run: |
          wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
          echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list
          apt-get update && apt-get install -y google-chrome-stable
      - name: Build Browser
        run: cargo build --release
      - name: Capture Screenshots
        run: |
          ./target/release/fusion-browser screenshot https://myapp.com --output before.png
```text

### Data Extraction

Extract structured data from websites:

```bash

# Extract all links

fusion-browser exec https://example.com --script \
  "Array.from(document.querySelectorAll('a')).map(a => ({text: a.textContent, href: a.href}))"

# Extract table data

fusion-browser exec https://example.com --script \
  "Array.from(document.querySelectorAll('table tr')).map(tr => Array.from(tr.querySelectorAll('td')).map(td => td.textContent))"
```text

## Best Practices

1. **Use Configuration Files**: For consistent behaviour across runs
2. **Save Screenshots**: For debugging and documentation
3. **Handle Timeouts**: Set appropriate timeouts for slow sites
4. **Test Different Modes**: Try various render modes to find what works best
5. **Enable Logging**: Use `--verbose` when troubleshooting
6. **Manage Sessions**: Clear old session data periodically

## Conclusion

The Fusion Terminal Browser provides powerful web browsing capabilities in the terminal. Whether you're automating web tasks, testing applications, or simply prefer terminal-based tools, it offers a robust and flexible solution.

For more advanced usage and API documentation, see the [Developer Guide](./developer_guide.md).