# Fusion Flux Engine - Quick Reference

## 🚀 Commands

### ✅ REQUIRED (use these)
```bash
fusion build              # Build project
fusion build --release    # Release build
fusion test               # Run tests
fusion run                # Build and run
fusion check              # Quick validation
```

### ❌ PROHIBITED (don't use these)
```bash
cargo build      # BLOCKED in strict mode
cargo test       # BLOCKED in strict mode
cargo run        # BLOCKED in strict mode
```

---

## ⚙️ Configuration

### Enable/Disable
```bash
# Enable Flux (default)
export FUSION_FLUX_ENABLED=true

# Disable Flux (not recommended)
export FUSION_FLUX_ENABLED=false
```

### Enforcement Level
```bash
# Strict mode (blocks cargo)
export FUSION_STRICT_MODE=true

# Warning mode (allows cargo with warning)
export FUSION_STRICT_MODE=false
```

### Emergency Fallback
```bash
# Allow cargo if Flux fails
export ALLOW_CARGO_FALLBACK=true

# No fallback (strict)
export ALLOW_CARGO_FALLBACK=false
```

---

## 🔧 Setup

### One-Command Setup
```powershell
# Windows
.\.scripts\setup-flux-enforcement.ps1

# Unix/Linux/macOS
chmod +x .scripts/setup-flux-enforcement.sh
./.scripts/setup-flux-enforcement.sh
```

### Manual Setup
```bash
# 1. Build Flux Engine
cd runtime
cargo build -p fusion_flux_resolve --release

# 2. Install Git hooks
git config core.hooksPath .githooks

# 3. Set environment
export FUSION_FLUX_ENABLED=true
export FUSION_STRICT_MODE=true
```

---

## 🆘 Emergency Override

### Temporary Disable (current session)
```powershell
# Windows
$env:FUSION_STRICT_MODE = 'false'
$env:ALLOW_CARGO_FALLBACK = 'true'

# Unix/Linux/macOS
export FUSION_STRICT_MODE=false
export ALLOW_CARGO_FALLBACK=true
```

### Bypass Git Hook
```bash
git commit --no-verify
```

---

## 📊 Status Check

### Verify Setup
```bash
# Check if Flux is built
ls runtime/target/release/fusion_flux_resolve*

# Check Git hooks
git config core.hooksPath

# Check environment
echo $FUSION_FLUX_ENABLED
echo $FUSION_STRICT_MODE
```

### Test Enforcement
```bash
# Should show policy violation
cargo build

# Should work
fusion build
```

---

## 🐛 Troubleshooting

### "Fusion command not found"
```bash
cd cmd/fusion
cargo build --release
export PATH="$PATH:$PWD/target/release"
```

### "Flux Engine not built"
```bash
cd runtime
cargo build -p fusion_flux_resolve --release
cargo test -p fusion_flux_resolve
```

### "Policy violation but I need cargo"
```bash
export FUSION_STRICT_MODE=false  # Temporary
```

---

## 📖 Documentation

- **Full Policy:** `BUILD_POLICY.md`
- **Implementation:** `docs/outputs/FLUX_ENFORCEMENT_IMPLEMENTATION.md`
- **What is Flux:** `docs/guides/WHAT_IS_FLUX_RESOLVE.md`
- **vs Cargo:** `docs/guides/FLUX_RESOLVE_VS_CARGO.md`

---

## 🎯 Quick Wins

### Speed
```
cargo build (cold):   Minutes
fusion build (cold):  Seconds

cargo build (cached): Seconds
fusion build (cached): Milliseconds
```

### Concurrency
```
Cargo:  1 build at a time
Flux:   100+ parallel builds
```

### Learning
```
Build #1:   500ms (learning)
Build #10:  50ms (optimized)
Build #100: <1ms (cached)
```

---

## ⚡ Key Features

- **Lock-Free:** No file locking, unlimited concurrency
- **Self-Learning:** VSIDS heuristics improve over time
- **GPU Accelerated:** 10-50× faster for large projects
- **Content-Addressable:** Shared cache across builds
- **Deterministic:** Same inputs = same outputs always

---

**Policy Version:** 1.0  
**Last Updated:** 2025-12-12  
**Enforcement:** STRICT
