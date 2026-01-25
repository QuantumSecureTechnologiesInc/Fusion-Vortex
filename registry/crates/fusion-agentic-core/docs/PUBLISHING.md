# Publishing to Global Registry

## Overview

This document describes the process for publishing the `fusion-agentic-core` crate to the global Rust package registry (crates.io) and the Fusion global package manager.

## Prerequisites

### 1. Crates.io Account

Create an account at [crates.io](https://crates.io) if you don't have one.

### 2. API Token

Get your API token from crates.io:
1. Visit https://crates.io/me
2. Generate a new token
3. Run: `cargo login <your-token>`

### 3. Package Verification

Ensure all requirements are met:

```bash

# Navigate to crate directory

cd registry/crates/fusion-agentic-core

# Check the package

cargo package --list

# Verify it builds

cargo build --release

# Run all tests

cargo test

# Check documentation

cargo doc --no-deps
```text

## Publication Steps

### Step 1: Version Check

Verify the version in `Cargo.toml`:

```toml
[package]
name = "fusion-agentic-core"
version = "0.1.0"  # Ensure this is correct
```text

### Step 2: Package Creation

Create the package:

```bash
cargo package
```text

This creates a `.crate` file in `target/package/`.

### Step 3: Dry Run

Test the publication without actually publishing:

```bash
cargo publish --dry-run
```text

Review the output for any errors or warnings.

### Step 4: Publish to Crates.io

Publish the crate:

```bash
cargo publish
```text

**Note**: This action is **permanent** and cannot be undone. The version number can never be reused.

### Step 5: Verify Publication

After a few minutes, verify at:
- https://crates.io/crates/fusion-agentic-core
- https://docs.rs/fusion-agentic-core

## Fusion Global Registry

### Integration

The Fusion global package manager automatically indexes from crates.io. Additional steps for Fusion-specific features:

### 1. Register with Fusion Registry

```bash

# Using Fusion CLI

fusion registry add fusion-agentic-core \
    --version 0.1.0 \
    --category ai-tools \
    --tags "ai,agentic,reasoning,vibe-coding,code-excellence"
```text

### 2. Add Metadata

Create `fusion-package.toml` in the crate root:

```toml
[package]
name = "fusion-agentic-core"
version = "0.1.0"
fusion_version = ">=0.2.0"

[features]
agentic_enhanced = true
vibe_coding = true
code_excellence = true

[metadata]
category = "ai-tools"
subcategory = "reasoning"
tags = ["ai", "agentic", "reasoning", "vibe-coding", "code-excellence"]
keywords = ["ai", "agentic", "reasoning", "vibe-coding", "code-excellence"]

[metadata.quality]
test_coverage = "80%"
documentation_score = "95%"
security_audit = "passed"

[compatibility]
rust_version = "1.80+"
platforms = ["windows", "linux", "macos"]
```text

### 3. Sync to Fusion Registry

```bash
fusion registry sync fusion-agentic-core
```text

## Post-Publication

### 1. Update Documentation

- Add badges to README.md
- Update CHANGELOG.md
- Create release notes

### 2. Announcement

Announce the release:
- GitHub release
- Project blog
- Community channels

### 3. Monitor

Check for:
- Download statistics
- Bug reports
- Feature requests
- Security advisories

## Version Management

### Semantic Versioning

Follow [SemVer](https://semver.org/):

- `0.1.0` → `0.1.1`: Bug fixes (patch)
- `0.1.0` → `0.2.0`: New features (minor)
- `0.1.0` → `1.0.0`: Breaking changes (major)

### Publishing Updates

```bash

# Update version in Cargo.toml


# Update CHANGELOG.md

# Build and test

cargo build --release
cargo test

# Publish

cargo publish
```text

## Troubleshooting

### Error: Already Published

**Problem**: Version already exists

**Solution**: Increment version number in `Cargo.toml`

### Error: Missing Metadata

**Problem**: Missing required fields in `Cargo.toml`

**Solution**: Add required fields:

```toml
[package]
name = "..."
version = "..."
authors = ["..."]
description = "..."
license = "..."
repository = "..."
```text

### Error: File Too Large

**Problem**: Package exceeds size limit

**Solution**:
- Add large files to `.gitignore`
- Use `.cargo_vcs_info.json` for VCS metadata
- Exclude unnecessary files in `Cargo.toml`:

  ```toml
  [package]
  exclude = ["tests/fixtures/*", "benches/data/*"]
```text

### Error: Documentation Build Failed

**Problem**: docs.rs build fails

**Solution**:
- Test locally: `cargo doc --no-deps`
- Check for platform-specific code
- Add build metadata if needed

## Crate Metadata

### Categories

Appropriate categories for this crate:
- `development-tools`
- `development-tools::code-generation`
- `development-tools::testing`

### Keywords

Recommended keywords (max 5):
- `ai`
- `agentic`
- `reasoning`
- `code-generation`
- `quality-assurance`

### Badges

Add to README.md:

```markdown
[![Crate](https://img.shields.io/crates/v/fusion-agentic-core.svg)](https://crates.io/crates/fusion-agentic-core)
[![Documentation](https://docs.rs/fusion-agentic-core/badge.svg)](https://docs.rs/fusion-agentic-core)
[![License](https://img.shields.io/crates/l/fusion-agentic-core.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/fusion-agentic-core.svg)](https://crates.io/crates/fusion-agentic-core)
```text

## Security

### Reporting Vulnerabilities

Create `SECURITY.md`:

```markdown

# Security Policy

## Reporting a Vulnerability

Please report security vulnerabilities to:
security@quantum-secure-technologies.com

Do not create public GitHub issues for security vulnerabilities.
```text

### Security Advisories

Monitor:
- RustSec Advisory Database
- GitHub Security Advisories
- Dependabot alerts

## Maintenance

### Regular Tasks

- Update dependencies quarterly
- Review and respond to issues
- Merge pull requests
- Update documentation
- Publish security patches promptly

### Deprecation Policy

For major changes:
1. Announce deprecation in version N
2. Maintain backward compatibility in N+1
3. Remove deprecated features in N+2

### Support Policy

- Latest version: Full support
- Previous minor: Security fixes only
- Older versions: Community support

---

## Checklist

Before publishing, ensure:

- [ ] Version updated in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] All tests pass
- [ ] Documentation builds
- [ ] Examples work
- [ ] README.md is current
- [ ] Licence files present
- [ ] No secrets in code
- [ ] Dry run successful
- [ ] Git tag created: `git tag v0.1.0`
- [ ] Git tag pushed: `git push --tags`

---

**Version**: 0.1.0
**Last Updated**: 2025-12-12
**Maintainer**: Quantum Secure Technologies Inc.