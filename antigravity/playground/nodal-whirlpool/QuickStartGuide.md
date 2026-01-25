# Fusion CLI - Quick Start Guide

Get started with Fusion in under 5 minutes!

## Installation

### From Source (Current Method)

```bash

# Clone the repository

git clone https://github.com/fusion-lang/fusion-cli.git
cd fusion-cli

# Build the CLI

cargo build --release

# Verify installation

./target/release/fusion --version
```text

### Add to PATH (Optional)

**Linux/macOS:**

```bash
export PATH="$PATH:$(pwd)/target/release"
```text

**Windows (PowerShell):**

```powershell
$env:PATH += ";$(Get-Location)\target\release"
```text

## Create Your First Project

```bash

# Create a new binary project

fusion new hello-fusion

# Navigate to the project

cd hello-fusion

# View the generated structure

tree .
```text

You should see:

```text
hello-fusion/
├── src/
│   └── main.fu
└── Fusion.toml
```text

## Your First Fusion Program

The generated `src/main.fu` contains:

```fusion
fn main() {
    println!("Hello from Fusion!");
}
```text

## Build and Run

```bash

# Build the project

fusion build

# Run the project

fusion run
```text

Output:

```text
Hello from Fusion!
```text

## Try AI-Powered Development

### Generate Code

```bash

# Generate a HTTP handler

fusion ai generate "create a HTTP GET handler that returns JSON" --target src/api.fu --preview-only

# Review the preview, then apply

fusion ai generate "create a HTTP GET handler that returns JSON" --target src/api.fu
```text

### Get Explanations

```bash

# Explain existing code

fusion ai explain src/main.fu --depth detailed
```text

### Interactive Assistant

```bash

# Start interactive session

fusion ai assist

# In the session, try:

fusion-ai> How do I create a struct?
fusion-ai> Generate a function to parse JSON
fusion-ai> exit
```text

## Essential Commands

### Development

```bash
fusion build              # Build project
fusion build --release    # Build with optimisations
fusion run                # Run project
fusion test               # Run tests
fusion check              # Type-check only
```text

### Code Quality

```bash
fusion fmt                # Format code
fusion fmt --check        # Check if formatted
fusion lint               # Run linter
fusion lint --fix         # Auto-fix issues
```text

### Package Management

```bash
fusion package add serde  # Add dependency
fusion package list       # List dependencies
fusion package update     # Update dependencies
```text

### AI Features

```bash
fusion ai assist          # Interactive assistant
fusion ai generate <desc> # Generate code
fusion ai review          # Review code
fusion ai tests <target>  # Generate tests
```text

## Configuration

### Project Configuration (Fusion.toml)

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2024"

[dependencies]

# Add dependencies here

```text

### AI Configuration

```bash

# View AI settings

fusion ai config --show

# Set default model

fusion ai config --model gpt-4

# Use offline mode

fusion ai generate "..." --ai-offline
```text

## Next Steps

1. **Read the User Guide**: `docs/guides/UserGuide.md`
2. **Explore Examples**: Check the `examples/` directory
3. **Try Advanced Features**: Profiling, debugging, deployment
4. **Join the Community**: Discord, Forum, GitHub Discussions

## Common Issues

### Build Fails

```bash

# Ensure Rust is up to date

rustup update stable

# Clean and rebuild

cargo clean
cargo build --workspace
```text

### AI Commands Not Working

The AI subsystem is currently in skeleton/mock mode. Full LLM integration coming in Phase 2.

### Need Help?

- 📚 Documentation: `docs/` directory
- 💬 Discord: [Join our server](https://discord.gg/fusion-lang)
- 🐛 Issues: [GitHub Issues](https://github.com/fusion-lang/fusion-cli/issues)

---

Welcome to Fusion! 🚀