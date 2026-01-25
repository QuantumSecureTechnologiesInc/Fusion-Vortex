# Fusion Build System Configuration

## Build Policy

**REQUIRED:** All Fusion projects must use **Fusion Flux Engine** for dependency resolution and builds.

### Strict Enforcement Mode

By default, the project operates in **strict mode** which prevents direct `cargo` usage.

```bash

# ❌ PROHIBITED

fusion build
fusion test
fusion run

# ✅ REQUIRED

fusion build
fusion test
fusion run
```text

## Environment Variables

### Core Configuration

```bash

# Enable/Disable Fusion Flux Engine

export FUSION_FLUX_ENABLED=true          # Default: true

# Strict enforcement mode

export FUSION_STRICT_MODE=true           # Default: true
                                         # false = warnings only

# Allow cargo fallback (emergency only)

export ALLOW_CARGO_FALLBACK=false        # Default: false
                                         # true = use cargo if Flux fails
```text

### Advanced Configuration

```bash

# GPU acceleration

export FUSION_CUDA_ENABLE=true           # Default: true

# Cache configuration

export FUSION_CACHE_PATH=./.fusion/cache_db
export FUSION_CACHE_MAX_SIZE_MB=1024

# Registry

export FUSION_REGISTRY_URL=https://registry.fusionlang.dev

# Telemetry

export FUSION_TELEMETRY_ENABLED=true
```text

## How It Works

### 1. Automatic Enforcement

The `.scripts/enforce-flux-build.*` scripts automatically intercept build commands:

```bash

# When you run:

fusion build

# The script:

1. Detects cargo usage ❌
2. Checks FUSION_STRICT_MODE
3. If strict: BLOCKS and shows error
4. If not strict: Warns and allows
```text

### 2. Proper Usage

```bash

# Use Fusion CLI (recommended)

fusion build              # Builds with Fusion Flux Engine
fusion build --release    # Release build
fusion test               # Run tests
fusion run                # Build and run
fusion check              # Fast check without building
```text

### 3. Emergency Override

If Fusion Flux Engine is not working:

```bash

# Temporary disable for this session

export FUSION_STRICT_MODE=false
export ALLOW_CARGO_FALLBACK=true

fusion build  # Now allowed (with warning)
```text

## CI/CD Integration

### GitHub Actions

```yaml

# .github/workflows/ci.yml

name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    env:
      FUSION_FLUX_ENABLED: true
      FUSION_STRICT_MODE: true

    steps:
      - uses: actions/checkout@v3

      - name: Build Fusion Flux Engine
        run: |
          cd runtime
          fusion build -p fusion_flux_resolve

      - name: Build Project (via Fusion)
        run: |
          # This will use Fusion Flux automatically
          fusion build

      - name: Run Tests
        run: fusion test
```text

### Local Development

Add to your shell profile (`.bashrc`, `.zshrc`, or PowerShell profile):

```bash

# Enforce Fusion Flux globally

export FUSION_FLUX_ENABLED=true
export FUSION_STRICT_MODE=true

# Alias cargo to enforcement script

alias cargo='$HOME/Projects/Fusion/scripts/enforce-flux-build.sh'
```text

## Policy Exemptions

### When Cargo is Allowed

1. **Building Fusion Flux itself:**

   ```bash
   cd runtime
   fusion build -p fusion_flux_resolve  # OK - building the engine
```text

2. **Emergency maintenance:**

   ```bash
   export FUSION_STRICT_MODE=false  # Temporary disable
   fusion build  # Allowed with warning
```text

3. **External dependencies:**

   ```bash
   # Building Rust crates that aren't Fusion projects
   cd external/some-rust-crate
   fusion build  # OK - not a Fusion project
```text

## Enforcement Levels

### Level 1: Informational (FUSION_STRICT_MODE=false)

```text
⚠️  WARNING: Using cargo directly
   Consider using 'fusion build' instead

Proceeding with fusion build...
```text

### Level 2: Strict (FUSION_STRICT_MODE=true, ALLOW_CARGO_FALLBACK=false)

```text
❌ POLICY VIOLATION: Direct cargo usage detected!

   Command attempted: fusion build

   ✅ USE INSTEAD:
      fusion build

Build blocked. Exiting.
```text

### Level 3: Strict with Fallback (FUSION_STRICT_MODE=true, ALLOW_CARGO_FALLBACK=true)

```text
⚠️  Fusion Flux Engine not available
   Falling back to cargo (ALLOW_CARGO_FALLBACK=true)

Proceeding with fusion build...
```text

## Rollout Plan

### Phase 1: Warning Mode (Current)

- `FUSION_STRICT_MODE=false` by default
- Warnings on cargo usage
- Education and migration

### Phase 2: Strict Mode (Target: v0.4.0)

- `FUSION_STRICT_MODE=true` by default
- Block cargo usage
- Fallback available if needed

### Phase 3: Full Enforcement (Target: v1.0.0)

- `FUSION_STRICT_MODE=true` mandatory
- No fallback allowed
- Cargo only for Flux itself

## Monitoring and Compliance

### Check Compliance

```bash

# Verify Flux is being used

.scripts/enforce-flux-build.ps1 check

# Expected output:

✅ Fusion Flux Engine: Available
✅ Strict Mode: Enabled
✅ Project Status: Compliant
```text

### Audit Builds

```bash

# Review build logs

cat .fusion/build-audit.log

# Sample output:

2025-12-12T13:40:00 [INFO] Build initiated
2025-12-12T13:40:00 [INFO] Fusion Flux Engine: v0.3.0
2025-12-12T13:40:00 [INFO] Cache hit rate: 95%
2025-12-12T13:40:01 [INFO] Build complete (1.2s)
```text

## Troubleshooting

### "Fusion Flux Engine not found"

```bash

# Build the engine first

cd runtime
fusion build -p fusion_flux_resolve

# Verify it's built

ls runtime/target/debug/fusion_flux_resolve*
```text

### "Policy violation" but I need cargo

```bash

# Temporary disable (this session only)

export FUSION_STRICT_MODE=false
fusion build

# Or allow fallback

export ALLOW_CARGO_FALLBACK=true
fusion build
```text

### "Fusion command not found"

```bash

# Build Fusion CLI

cd cmd/fusion
fusion build --release

# Add to PATH

export PATH="$PATH:$PWD/target/release"

# Or create symlink

ln -s $PWD/target/release/fusion /usr/local/bin/fusion
```text

## FAQ

**Q: Why enforce Fusion Flux?**
A: Ensures consistent, fast, parallel builds across all team members and CI/CD.

**Q: What if Flux has a bug?**
A: Use `ALLOW_CARGO_FALLBACK=true` temporarily while the bug is fixed.

**Q: Can I use both Fusion and cargo?**
A: No in strict mode. Fusion Flux handles everything cargo does, but better.

**Q: How do I migrate existing builds?**
A: Replace `fusion build` → `fusion build`. Flux handles the rest automatically.

**Q: Does this break IDE integration?**
A: No. Configure your IDE to use `fusion build` instead of `fusion build`.

## Support

For issues or questions:
- GitHub Issues: `https://github.com/QuantumSecureTechnologiesInc/Fusion/issues`
- Documentation: `docs/guides/`
- Build Policy: This file

---

**Last Updated:** 2025-12-12
**Policy Version:** 1.0
**Enforcement:** STRICT