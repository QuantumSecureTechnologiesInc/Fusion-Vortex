#!/bin/bash
# Build Native Fusion Compiler Script
# 
# This script builds the Pure Fusion native compiler for self-hosting.
# It compiles the Pure Fusion compiler components to native machine code.

set -e

echo "🚀 Building Pure Fusion Native Compiler"
echo "========================================"

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="$WORKSPACE_ROOT/target"
RELEASE_DIR="$TARGET_DIR/release"
BUILD_DIR="$TARGET_DIR/build"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    echo -e "${BLUE}[BUILD]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check if Cargo is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Cargo first."
        exit 1
    fi
    
    # Check if LLVM is installed
    if ! command -v llvm-config &> /dev/null; then
        print_warning "LLVM is not installed. Some features may not work."
    fi
    
    # Check if C compiler is available
    if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null && ! command -v clang &> /dev/null; then
        print_error "No C compiler found. Please install gcc, clang, or another C compiler."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Create directories
setup_directories() {
    print_status "Setting up build directories..."
    
    mkdir -p "$TARGET_DIR"
    mkdir -p "$RELEASE_DIR"
    mkdir -p "$BUILD_DIR"
    
    print_success "Build directories created"
}

# Use existing bootstrap compiler
use_existing_bootstrap_compiler() {
    print_status "Using existing bootstrap compiler..."
    
    BOOTSTRAP_COMPILER="$WORKSPACE_ROOT/bin/fuc"
    
    if [ ! -f "$BOOTSTRAP_COMPILER" ]; then
        print_error "Existing bootstrap compiler not found at $BOOTSTRAP_COMPILER"
        exit 1
    fi
    
    print_success "Existing bootstrap compiler found at $BOOTSTRAP_COMPILER"
}

# Compile Pure Fusion compiler components
compile_pure_fusion_components() {
    print_status "Compiling Pure Fusion compiler components..."
    
    # Compile the main compiler
    cd "$WORKSPACE_ROOT"
    
    # Use the existing bootstrap compiler to compile Pure Fusion components
    BOOTSTRAP_COMPILER="$WORKSPACE_ROOT/bin/fuc"
    
    if [ ! -f "$BOOTSTRAP_COMPILER" ]; then
        print_error "Bootstrap compiler not found at $BOOTSTRAP_COMPILER"
        exit 1
    fi
    
    # Compile the Pure Fusion compiler
    print_status "Compiling Pure Fusion compiler..."
    "$BOOTSTRAP_COMPILER" \
        "crates/fuc/src/pure_fusion_compiler_minimal.fu" \
        --output "$RELEASE_DIR/fuc_native" \
        --opt-level 2 \
        --emit-bin \
        --target "x86_64-pc-windows-msvc"
    
    if [ $? -eq 0 ]; then
        print_success "Pure Fusion compiler compiled successfully"
    else
        print_error "Failed to compile Pure Fusion compiler"
        exit 1
    fi
    
    # Compile the CLI
    print_status "Compiling Pure Fusion CLI..."
    "$BOOTSTRAP_COMPILER" \
        "registry/crates/fusion-core/src/bin/fuc_cli.fu" \
        --output "$RELEASE_DIR/fuc_cli_native" \
        --opt-level 2 \
        --emit-bin \
        --target "x86_64-pc-windows-msvc"
    
    if [ $? -eq 0 ]; then
        print_success "Pure Fusion CLI compiled successfully"
    else
        print_error "Failed to compile Pure Fusion CLI"
        exit 1
    fi
}

# Compile standard library
compile_stdlib() {
    print_status "Compiling standard library..."
    
    BOOTSTRAP_COMPILER="$WORKSPACE_ROOT/bin/fuc"
    
    # Find and compile stdlib
    STDLIB_SOURCES=(
        "$WORKSPACE_ROOT/registry/crates/std/src/lib.fu"
        "$WORKSPACE_ROOT/stdlib/src/lib.fu"
    )
    
    STDLIB_SOURCE=""
    for source in "${STDLIB_SOURCES[@]}"; do
        if [ -f "$source" ]; then
            STDLIB_SOURCE="$source"
            break
        fi
    done
    
    if [ -z "$STDLIB_SOURCE" ]; then
        print_warning "Standard library source not found, skipping stdlib compilation"
        return 0
    fi
    
    "$BOOTSTRAP_COMPILER" \
        "$STDLIB_SOURCE" \
        --output "$RELEASE_DIR/std.o" \
        --lib \
        --opt-level 2
    
    if [ $? -eq 0 ]; then
        print_success "Standard library compiled successfully"
    else
        print_warning "Failed to compile standard library"
    fi
}

# Compile runtime components
compile_runtime() {
    print_status "Compiling runtime components..."
    
    RUNTIME_DIR="$BUILD_DIR/runtime"
    mkdir -p "$RUNTIME_DIR"
    
    # Compile panic runtime
    cat > "$RUNTIME_DIR/panic.c" << 'EOF'
#include <stdio.h>
#include <stdlib.h>

void panic(const char* msg) {
    fprintf(stderr, "PANIC: %s\n", msg);
    fflush(stderr);
    abort();
}

void panic_with_location(const char* msg, const char* file, int line) {
    fprintf(stderr, "PANIC at %s:%d: %s\n", file, line, msg);
    fflush(stderr);
    abort();
}
EOF

    cc -c "$RUNTIME_DIR/panic.c" -o "$RUNTIME_DIR/panic.o"
    
    if [ $? -eq 0 ]; then
        print_success "Panic runtime compiled successfully"
    else
        print_error "Failed to compile panic runtime"
        exit 1
    fi
    
    # Compile ARC runtime if available
    ARC_RUNTIME_SOURCES=(
        "$WORKSPACE_ROOT/src/stdlib/arc_runtime.c"
        "$WORKSPACE_ROOT/Source Files/Automatic Reference Counting (ARC)/runtime.c"
    )
    
    for source in "${ARC_RUNTIME_SOURCES[@]}"; do
        if [ -f "$source" ]; then
            print_status "Compiling ARC runtime from $source"
            cc -c "$source" -o "$RUNTIME_DIR/arc_runtime.o"
            
            if [ $? -eq 0 ]; then
                print_success "ARC runtime compiled successfully"
                break
            else
                print_warning "Failed to compile ARC runtime from $source"
            fi
        fi
    done
}

# Link final executables
link_executables() {
    print_status "Linking final executables..."
    
    BOOTSTRAP_COMPILER="$WORKSPACE_ROOT/bin/fuc"
    RUNTIME_DIR="$BUILD_DIR/runtime"
    
    # Link fuc_native
    if [ -f "$RELEASE_DIR/fuc_native" ]; then
        print_status "Linking fuc_native..."
        
        # Check if we have stdlib and runtime objects
        STD_LIB=""
        if [ -f "$RELEASE_DIR/std.o" ]; then
            STD_LIB="$RELEASE_DIR/std.o"
        fi
        
        RUNTIME_OBJS=""
        if [ -f "$RUNTIME_DIR/panic.o" ]; then
            RUNTIME_OBJS="$RUNTIME_DIR/panic.o"
        fi
        
        if [ -f "$RUNTIME_DIR/arc_runtime.o" ]; then
            RUNTIME_OBJS="$RUNTIME_OBJS $RUNTIME_DIR/arc_runtime.o"
        fi
        
        # Link the executable
        cc -o "$RELEASE_DIR/fuc_native_final" \
           "$RELEASE_DIR/fuc_native" \
           $STD_LIB \
           $RUNTIME_OBJS \
           -no-pie -lc
        
        if [ $? -eq 0 ]; then
            mv "$RELEASE_DIR/fuc_native_final" "$RELEASE_DIR/fuc_native"
            print_success "fuc_native linked successfully"
        else
            print_error "Failed to link fuc_native"
            exit 1
        fi
    fi
    
    # Link fuc_cli_native
    if [ -f "$RELEASE_DIR/fuc_cli_native" ]; then
        print_status "Linking fuc_cli_native..."
        
        # Link the executable
        cc -o "$RELEASE_DIR/fuc_cli_native_final" \
           "$RELEASE_DIR/fuc_cli_native" \
           $STD_LIB \
           $RUNTIME_OBJS \
           -no-pie -lc
        
        if [ $? -eq 0 ]; then
            mv "$RELEASE_DIR/fuc_cli_native_final" "$RELEASE_DIR/fuc_cli_native"
            print_success "fuc_cli_native linked successfully"
        else
            print_error "Failed to link fuc_cli_native"
            exit 1
        fi
    fi
}

# Run self-hosting tests
run_self_hosting_tests() {
    print_status "Running self-hosting tests..."
    
    # Test fuc_native
    if [ -f "$RELEASE_DIR/fuc_native" ]; then
        print_status "Testing fuc_native..."
        
        # Create a simple test file
        echo 'fn main() { print("Hello from self-hosted compiler!"); }' > "$BUILD_DIR/test.fu"
        
        # Try to compile it with the self-hosted compiler
        "$RELEASE_DIR/fuc_native" \
            --input "$BUILD_DIR/test.fu" \
            --output "$BUILD_DIR/test_output" \
            --emit-bin
        
        if [ $? -eq 0 ]; then
            print_success "fuc_native self-hosting test passed"
        else
            print_warning "fuc_native self-hosting test failed"
        fi
        
        # Clean up test file
        rm -f "$BUILD_DIR/test.fu" "$BUILD_DIR/test_output"
    fi
    
    # Test fuc_cli_native
    if [ -f "$RELEASE_DIR/fuc_cli_native" ]; then
        print_status "Testing fuc_cli_native..."
        
        "$RELEASE_DIR/fuc_cli_native" --version
        
        if [ $? -eq 0 ]; then
            print_success "fuc_cli_native self-hosting test passed"
        else
            print_warning "fuc_cli_native self-hosting test failed"
        fi
    fi
}

# Create installation script
create_installation_script() {
    print_status "Creating installation script..."
    
    cat > "$RELEASE_DIR/install_native_compiler.sh" << EOF
#!/bin/bash
# Installation script for Pure Fusion Native Compiler

set -e

INSTALL_DIR="\$HOME/.local/bin"
mkdir -p "\$INSTALL_DIR"

echo "Installing Pure Fusion Native Compiler..."

# Copy executables
if [ -f "$RELEASE_DIR/fuc_native" ]; then
    cp "$RELEASE_DIR/fuc_native" "\$INSTALL_DIR/fuc"
    chmod +x "\$INSTALL_DIR/fuc"
    echo "Installed fuc to \$INSTALL_DIR/fuc"
fi

if [ -f "$RELEASE_DIR/fuc_cli_native" ]; then
    cp "$RELEASE_DIR/fuc_cli_native" "\$INSTALL_DIR/fuc-cli"
    chmod +x "\$INSTALL_DIR/fuc-cli"
    echo "Installed fuc-cli to \$INSTALL_DIR/fuc-cli"
fi

echo "Installation complete!"
echo "Make sure \$HOME/.local/bin is in your PATH"
EOF

    chmod +x "$RELEASE_DIR/install_native_compiler.sh"
    print_success "Installation script created at $RELEASE_DIR/install_native_compiler.sh"
}

# Main build process
main() {
    echo "Starting Pure Fusion Native Compiler build process..."
    echo
    
    check_prerequisites
    setup_directories
    use_existing_bootstrap_compiler
    compile_pure_fusion_components
    compile_stdlib
    compile_runtime
    link_executables
    run_self_hosting_tests
    create_installation_script
    
    echo
    print_success "Pure Fusion Native Compiler build completed successfully!"
    echo
    echo "Generated files:"
    echo "  - $RELEASE_DIR/fuc_native (main compiler)"
    echo "  - $RELEASE_DIR/fuc_cli_native (CLI interface)"
    echo "  - $RELEASE_DIR/install_native_compiler.sh (installation script)"
    echo
    echo "To install the compiler system-wide, run:"
    echo "  $RELEASE_DIR/install_native_compiler.sh"
    echo
    echo "To test the self-hosted compiler:"
    echo "  echo 'fn main() { print(\"Hello World!\"); }' > test.fu"
    echo "  $RELEASE_DIR/fuc_native --input test.fu --output test --emit-bin"
    echo "  ./test"
}

# Run main function
main "$@"