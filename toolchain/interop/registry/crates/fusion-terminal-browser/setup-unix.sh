#!/bin/bash

# Fusion Terminal Browser - Linux/Mac Setup Script
# Downloads Chrome for Testing and sets up the environment

echo "==================================================="
echo " Fusion Terminal Browser - Linux/Mac Setup"
echo "==================================================="
echo ""

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)     PLATFORM="linux64";;
    Darwin*)    PLATFORM="mac-x64";;
    *)          echo "Unsupported OS: $OS"; exit 1;;
esac

echo "Detected platform: $PLATFORM"

# Create bin directory
mkdir -p bin
mkdir -p logs
mkdir -p chrome_profile

# Check if Chrome already exists
if [ "$PLATFORM" = "mac-x64" ]; then
    CHROME_PATH="./bin/chrome-$PLATFORM/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing"
else
    CHROME_PATH="./bin/chrome-$PLATFORM/chrome"
fi

if [ -f "$CHROME_PATH" ]; then
    echo "Chrome for Testing already installed at: $CHROME_PATH"
    echo "Skipping download..."
else
    echo "Downloading Chrome for Testing..."
    
    # Get latest version
    LATEST_URL="https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions.json"
    VERSION=$(curl -s $LATEST_URL | grep -Po '"version":\s*"\K[^"]+' | head -1)
    
    if [ -z "$VERSION" ]; then
        # Fallback version
        VERSION="131.0.6778.85"
        echo "Using fallback version: $VERSION"
    else
        echo "Latest version: $VERSION"
    fi
    
    # Construct download URL
    if [ "$PLATFORM" = "mac-x64" ]; then
        DOWNLOAD_URL="https://storage.googleapis.com/chrome-for-testing-public/$VERSION/mac-x64/chrome-mac-x64.zip"
    else
        DOWNLOAD_URL="https://storage.googleapis.com/chrome-for-testing-public/$VERSION/linux64/chrome-linux64.zip"
    fi
    
    echo "Download URL: $DOWNLOAD_URL"
    
    # Download
    curl -# -L "$DOWNLOAD_URL" -o bin/chrome.zip
    
    if [ $? -ne 0 ]; then
        echo "Error downloading Chrome!"
        echo "Please download Chrome for Testing manually from:"
        echo "https://googlechromelabs.github.io/chrome-for-testing/"
        exit 1
    fi
    
    echo "Download complete!"
    
    # Extract
    echo "Extracting Chrome..."
    cd bin
    unzip -q chrome.zip
    rm chrome.zip
    cd ..
    
    # Make executable (Linux)
    if [ "$PLATFORM" = "linux64" ]; then
        chmod +x "$CHROME_PATH"
    fi
    
    echo "Extraction complete!"
fi

# Set environment variable
export CHROME_PATH="$CHROME_PATH"

echo ""
echo "Chrome installed at: $CHROME_PATH"
echo ""
echo "==================================================="
echo " Setup Complete!"
echo "==================================================="
echo ""
echo "To persist CHROME_PATH, add to your shell profile:"
echo "  export CHROME_PATH=\"$(pwd)/$CHROME_PATH\""
echo ""
echo "For bash: ~/.bashrc"
echo "For zsh:  ~/.zshrc"
echo ""
echo "Build and run:"
echo "  fusion build --release"
echo "  fusion run --release -- --url https://webgpu.github.io/webgpu-samples/"
echo ""
