#!/usr/bin/env bash
# Fusion Build Policy Enforcement (Unix/Linux/macOS)
# This script ensures Fusion Flux Engine is used instead of cargo

set -e

# Configuration
FUSION_FLUX_ENABLED="${FUSION_FLUX_ENABLED:-true}"
ALLOW_CARGO_FALLBACK="${ALLOW_CARGO_FALLBACK:-false}"
FUSION_STRICT_MODE="${FUSION_STRICT_MODE:-true}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Print functions
print_success() { echo -e "${GREEN}$1${NC}"; }
print_error() { echo -e "${RED}$1${NC}"; }
print_warning() { echo -e "${YELLOW}$1${NC}"; }
print_info() { echo -e "${CYAN}$1${NC}"; }

# Check if Fusion Flux Engine is available
check_flux_available() {
    local flux_path="runtime/crates/fusion_flux_resolve"
    
    if [ -d "$flux_path" ]; then
        if [ -f "$flux_path/target/debug/libfusion_flux_resolve.so" ] || \
           [ -f "$flux_path/target/debug/libfusion_flux_resolve.dylib" ] || \
           [ -f "$flux_path/target/release/libfusion_flux_resolve.so" ] || \
           [ -f "$flux_path/target/release/libfusion_flux_resolve.dylib" ]; then
            return 0
        fi
    fi
    return 1
}

# Detect if user is trying to run cargo directly
check_cargo_attempt() {
    local cmd="$1"
    case "$cmd" in
        cargo|cargo-build|cargo-test|cargo-run|cargo-check)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

# Main enforcement logic
print_info "╔══════════════════════════════════════════════════════════╗"
print_info "║        FUSION BUILD POLICY ENFORCEMENT                   ║"
print_info "╚══════════════════════════════════════════════════════════╝"
echo ""

# Check if Fusion Flux is enabled
if [ "$FUSION_FLUX_ENABLED" != "true" ]; then
    print_warning "⚠️  Fusion Flux Engine is DISABLED"
    print_warning "   Set FUSION_FLUX_ENABLED=true to enable"
    print_warning "   Falling back to cargo..."
    exec cargo "$@"
    exit $?
fi

# Check if this is a cargo attempt
COMMAND="${1:-}"
if check_cargo_attempt "$COMMAND"; then
    if [ "$FUSION_STRICT_MODE" = "true" ]; then
        print_error "❌ POLICY VIOLATION: Direct cargo usage detected!"
        echo ""
        print_error "   Command attempted: $COMMAND"
        echo ""
        print_error "   FUSION BUILD POLICY:"
        print_error "   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        print_error "   Fusion projects must use Fusion Flux Engine."
        print_error "   Direct cargo usage is prohibited."
        echo ""
        print_error "   ✅ USE INSTEAD:"
        print_error "      fusion build    (instead of 'cargo build')"
        print_error "      fusion test     (instead of 'cargo test')"
        print_error "      fusion run      (instead of 'cargo run')"
        print_error "      fusion check    (instead of 'cargo check')"
        echo ""
        print_error "   ⚙️  TO DISABLE ENFORCEMENT (emergency only):"
        print_error "      export FUSION_STRICT_MODE=false"
        print_error "      export ALLOW_CARGO_FALLBACK=true"
        echo ""
        exit 1
    fi
fi

# Check if Flux is available
if ! check_flux_available; then
    print_warning "⚠️  Fusion Flux Engine not built yet"
    echo ""
    print_info "   Building Fusion Flux Engine first..."
    
    (cd runtime && cargo build -p fusion_flux_resolve)
    
    if [ $? -eq 0 ]; then
        print_success "✅ Fusion Flux Engine built successfully"
    else
        print_error "❌ Failed to build Fusion Flux Engine"
        
        if [ "$ALLOW_CARGO_FALLBACK" = "true" ]; then
            print_warning "   Fallback to cargo enabled - proceeding with cargo"
            exec cargo "$@"
            exit $?
        else
            print_error "   Cargo fallback disabled. Build cannot continue."
            print_error "   To enable fallback: export ALLOW_CARGO_FALLBACK=true"
            exit 1
        fi
    fi
fi

print_success "✅ Fusion Flux Engine is available"
echo ""
print_info "🚀 Routing build through Fusion Flux Engine..."
echo ""

# TODO: Once Fusion CLI is ready, route through it
print_warning "⚠️  Fusion CLI integration pending"
print_warning "   Currently using cargo as build backend"
print_warning "   Fusion Flux will handle dependency resolution"
echo ""

# Execute cargo with Fusion Flux awareness
exec cargo "$@"
