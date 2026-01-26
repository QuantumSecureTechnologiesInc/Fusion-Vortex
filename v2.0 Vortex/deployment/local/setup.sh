#!/bin/bash
# HyperCycle Vortex v2.0.2 - Local Development Setup
# For Linux and macOS

set -e

echo "=== HyperCycle Vortex v2.0.2 - Local Setup ==="
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=Linux;;
    Darwin*)    PLATFORM=Mac;;
    *)          PLATFORM="UNKNOWN:${OS}"
esac

echo "Platform detected: $PLATFORM"
echo ""

# Check prerequisites
echo "Checking prerequisites..."

# Check CMake
if ! command -v cmake &> /dev/null; then
    echo "❌ CMake not found. Please install CMake 3.21+"
    exit 1
fi
echo "✅ CMake found: $(cmake --version | head -n1)"

# Check GCC or Clang
if command -v gcc &> /dev/null; then
    echo "✅ GCC found: $(gcc --version | head -n1)"
elif command -v clang &> /dev/null; then
    echo "✅ Clang found: $(clang --version | head -n1)"
else
    echo "❌ No C compiler found. Please install GCC or Clang"
    exit 1
fi

# Check OpenSSL (optional)
if command -v openssl &> /dev/null; then
    echo "✅ OpenSSL found: $(openssl version)"
else
    echo "⚠️  OpenSSL not found (optional - will use internal SHA-3)"
fi

echo ""
echo "Installing dependencies..."

# Install dependencies based on platform
if [ "$PLATFORM" = "Linux" ]; then
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y build-essential cmake libssl-dev
    elif command -v yum &> /dev/null; then
        sudo yum install -y gcc gcc-c++ cmake openssl-devel
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y gcc gcc-c++ cmake openssl-devel
    fi
elif [ "$PLATFORM" = "Mac" ]; then
    if command -v brew &> /dev/null; then
        brew install cmake openssl
    else
        echo "⚠️  Homebrew not found. Please install dependencies manually."
    fi
fi

echo ""
echo "Configuring build..."

# Navigate to project root
cd "$(dirname "$0")/../.."

# Create build directory
mkdir -p build
cd build

# Configure with CMake
cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DHC_BUILD_TESTS=ON \
    -DHC_ENABLE_CUDA=OFF \
    -DHC_ENABLE_ROCM=OFF

echo ""
echo "Building..."

# Build
cmake --build . --parallel $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

echo ""
echo "Running tests..."

# Run tests
ctest --output-on-failure

echo ""
echo "✅ Setup complete!"
echo ""
echo "Binaries are in: $(pwd)"
echo ""
echo "To run the dashboard:"
echo "  ./vortex_dashboard"
echo ""
echo "To run the secure dashboard:"
echo "  ./secure_dashboard"
echo ""
