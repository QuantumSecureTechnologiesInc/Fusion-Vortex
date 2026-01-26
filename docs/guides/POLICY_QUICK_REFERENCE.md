# Fusion v2.0 Vortex Policy System - Quick Reference Guide

## Overview

The Fusion Policy System provides capability-based security for VS Code extensions and MCP tools. Every privileged operation requires explicit capability grants.

---

## Core Concepts

### 1. Capabilities

Permissions that extensions can request:

| Capability          | Risk   | Description                  |
| ------------------- | ------ | ---------------------------- |
| `FilesystemRead`    | Low    | Read files and directories   |
| `FilesystemWrite`   | High   | Create, modify, delete files |
| `NetworkOutbound`   | Medium | Make HTTP/network requests   |
| `ProcessSpawn`      | High   | Execute child processes      |
| `CredentialRead`    | High   | Read stored credentials      |
| `CredentialWrite`   | High   | Store credentials            |
| `WorkspaceInspect`  | Low    | Read workspace metadata      |
| `LspAccess`         | Medium | Access language servers      |
| `TerminalAccess`    | High   | Access terminal/shell        |
| `ClipboardAccess`   | Low    | Access clipboard             |
| `EnvironmentAccess` | Low    | Read environment variables   |

### 2. Trust Levels

Publisher trust classification:

| Level         | Icon | Meaning                             | Auto-Grant Safe?  |
| ------------- | ---- | ----------------------------------- | ----------------- |
| `Verified`    | ✅    | Cryptographic signatures            | Yes (High risk)   |
| `Trusted`     | 🔐    | Known publisher (Google, Microsoft) | Yes (Medium risk) |
| `Community`   | 👥    | Community-reviewed                  | No                |
| `Unverified`  | ⚠️    | Unknown source                      | No                |
| `UserTrusted` | 🔧    | Local development                   | Yes (High risk)   |

### 3. Enforcement Modes

How violations are handled:

| Mode       | Behavior                 | Use Case         |
| ---------- | ------------------------ | ---------------- |
| `Strict`   | Hard fail on violations  | Production       |
| `Warn`     | Log violations but allow | Migration        |
| `Disabled` | No enforcement           | Development only |

---

## Rust API

### Creating a Manifest

```rust
use fusion_policy::{ExtensionManifest, TrustLevel, Capability};

// Create new manifest
let mut manifest = ExtensionManifest::new(
    "google.gemini-code-assist",
    TrustLevel::Trusted,
);

// Add capabilities with justification
manifest.add_capability(
    Capability::NetworkOutbound,
    "Makes API calls to Gemini service",
);

manifest.add_capability(
    Capability::WorkspaceInspect,
    "Analyzes project structure for context",
);

// Save to disk
let extensions_dir = dirs::home_dir()
    .unwrap()
    .join(".fusion/extensions");
manifest.save(&extensions_dir)?;
```text

### Loading a Manifest

```rust
use fusion_policy::ExtensionManifest;

let extensions_dir = dirs::home_dir()
    .unwrap()
    .join(".fusion/extensions");

let manifest = ExtensionManifest::load(
    &extensions_dir,
    "google.gemini-code-assist",
)?;

println!("Trust level: {}", manifest.trust);
println!("Capabilities: {:?}", manifest.capabilities);
```text

### Enforcing Capabilities

```rust
use fusion_policy::{PolicyEnforcer, Capability, EnforcementMode};

// Create enforcer (strict mode by default)
let enforcer = PolicyEnforcer::strict();

// Define what the extension is allowed to do
let allowed = vec![
    Capability::NetworkOutbound,
    Capability::WorkspaceInspect,
];

// Check if operation is permitted
let requested = vec![Capability::NetworkOutbound];
match enforcer.enforce(&requested, &allowed) {
    Ok(()) => println!("✅ Operation allowed"),
    Err(e) => println!("❌ Violation: {}", e),
}

// Single capability check
enforcer.check_capability(
    &Capability::FilesystemWrite,
    &allowed,
)?; // Error: FilesystemWrite not allowed
```text

### Trust Verification

```rust
use fusion_policy::{TrustVerifier, TrustLevel};

let verifier = TrustVerifier::new();

// Verify extension by ID
let level = verifier.verify_extension(
    "google.gemini-code-assist",
    "marketplace",
);
println!("Trust level: {}", level); // 🔐 Trusted

// Get recommended trust by publisher
let recommended = verifier.recommended_trust("microsoft");
assert_eq!(recommended, TrustLevel::Verified);
```text

### Warn Mode (for Migration)

```rust
use fusion_policy::{PolicyEnforcer, Capability, EnforcementMode};

let enforcer = PolicyEnforcer::warn_only();

let requested = vec![Capability::FilesystemWrite];
let allowed = vec![]; // Nothing allowed

// This succeeds but logs a warning
enforcer.enforce(&requested, &allowed)?; // ⚠️  WARNING logged
```text

---

## CLI Commands (Future)

Once integrated, these commands will be available:

### View Extension Capabilities

```bash

# Show what an extension can do

fusion policy show google.gemini-code-assist

# Output:


# Extension: google.gemini-code-assist


# Trust: 🔐 Trusted


# Capabilities:


#   - NetworkOutbound (Medium risk)


#     Reason: Makes API calls to Gemini service


#   - WorkspaceInspect (Low risk)


#     Reason: Analyzes project structure

```text

### Grant/Revoke Capabilities

```bash

# Grant a capability

fusion policy grant google.gemini-code-assist FilesystemRead

# Revoke a capability

fusion policy revoke google.gemini-code-assist NetworkOutbound
```text

### Audit All Extensions

```bash

# Audit all installed extensions

fusion policy audit

# Output:


# Auditing 5 extensions...


#


# google.gemini-code-assist (🔐 Trusted)


#   ✅ NetworkOutbound


#   ✅ WorkspaceInspect


#


# saoudrizwan.cline (⚠️  Unverified)


#   ⚠️  FilesystemWrite (High risk, unverified publisher)


#   ⚠️  ProcessSpawn (High risk, unverified publisher)

```text

### Set Enforcement Mode

```bash

# Set to strict mode (production)

fusion policy mode strict

# Set to warn mode (migration)

fusion policy mode warn

# Disable enforcement (development only)

fusion policy mode disabled
```text

### Check Compatibility

```bash

# Check if extension capabilities are compatible

fusion policy check google.gemini-code-assist

# Output:


# ✅ All capabilities are valid


# ⚠️  3 high-risk capabilities require user approval:


#    - FilesystemWrite


#    - ProcessSpawn


#    - CredentialWrite

```text

---

## Manifest File Format

**Location**: `~/.fusion/extensions/<publisher>.<name>/capabilities.json`

```json
{
  "extension": "google.gemini-code-assist",
  "trust": "Trusted",
  "capabilities": [
    "NetworkOutbound",
    "CredentialRead",
    "WorkspaceInspect"
  ],
  "justifications": [
    {
      "capability": "NetworkOutbound",
      "reason": "Makes API calls to Gemini service"
    },
    {
      "capability": "CredentialRead",
      "reason": "Requires API key for authentication"
    },
    {
      "capability": "WorkspaceInspect",
      "reason": "Analyzes project structure for context"
    }
  ],
  "version": 1
}
```text

---

## Integration Examples

### ExtensionHost Integration

```rust
use fusion_policy::{PolicyEnforcer, ExtensionManifest, Capability};
use std::collections::HashMap;

pub struct ExtensionHost {
    enforcer: PolicyEnforcer,
    manifests: HashMap<String, ExtensionManifest>,
}

impl ExtensionHost {
    pub fn new() -> Self {
        Self {
            enforcer: PolicyEnforcer::strict(),
            manifests: HashMap::new(),
        }
    }

    pub fn load_manifest(&mut self, ext_id: &str) -> Result<()> {
        let manifest = ExtensionManifest::load(
            &self.extensions_dir(),
            ext_id,
        )?;
        self.manifests.insert(ext_id.to_string(), manifest);
        Ok(())
    }

    pub async fn check_capability(
        &self,
        extension_id: &str,
        capability: Capability,
    ) -> Result<()> {
        let manifest = self.manifests.get(extension_id)
            .ok_or_else(|| anyhow!("No manifest for {}", extension_id))?;

        self.enforcer.check_capability(&capability, &manifest.capabilities)
    }

    // Gate filesystem reads
    pub async fn read_file(&self, ext_id: &str, path: &str) -> Result<String> {
        self.check_capability(ext_id, Capability::FilesystemRead).await?;
        std::fs::read_to_string(path)
    }

    // Gate filesystem writes
    pub async fn write_file(&self, ext_id: &str, path: &str, content: &str) -> Result<()> {
        self.check_capability(ext_id, Capability::FilesystemWrite).await?;
        std::fs::write(path, content)
    }

    // Gate network requests
    pub async fn http_get(&self, ext_id: &str, url: &str) -> Result<String> {
        self.check_capability(ext_id, Capability::NetworkOutbound).await?;
        reqwest::get(url).await?.text().await
    }
}
```text

### Node.js Bridge Integration

```rust
// In crates/vscode-runtime/src/node_bridge/fs.rs

impl NodeFsBridge {
    pub async fn read_file_sync(
        &self,
        extension_id: &str,
        path: &str,
    ) -> Result<String> {
        // Enforce capability BEFORE operation
        self.host.check_capability(
            extension_id,
            Capability::FilesystemRead,
        ).await?;

        // Proceed with actual operation
        std::fs::read_to_string(path)
    }

    pub async fn write_file_sync(
        &self,
        extension_id: &str,
        path: &str,
        data: &str,
    ) -> Result<()> {
        // Enforce capability BEFORE operation
        self.host.check_capability(
            extension_id,
            Capability::FilesystemWrite,
        ).await?;

        // Proceed with actual operation
        std::fs::write(path, data)
    }
}
```text

---

## Best Practices

### 1. Principle of Least Privilege

Only request capabilities that are absolutely necessary:

```rust
// ❌ BAD: Requesting all capabilities
let manifest = ExtensionManifest::with_capabilities(
    "my-extension",
    TrustLevel::UserTrusted,
    Capability::all(), // Don't do this!
);

// ✅ GOOD: Only what's needed
let manifest = ExtensionManifest::with_capabilities(
    "my-extension",
    TrustLevel::UserTrusted,
    vec![
        Capability::WorkspaceInspect, // Only inspect, no writes
    ],
);
```text

### 2. Always Justify Capabilities

Help users understand why capabilities are needed:

```rust
manifest.add_capability(
    Capability::NetworkOutbound,
    "Makes API calls to check for updates", // Clear explanation
);
```text

### 3. Use Appropriate Trust Levels

Don't over-trust extensions:

```rust
// ❌ BAD: Trusting unknown publishers
let manifest = ExtensionManifest::new(
    "unknown.extension",
    TrustLevel::Verified, // Don't do this!
);

// ✅ GOOD: Conservative trust
let manifest = ExtensionManifest::new(
    "unknown.extension",
    TrustLevel::Unverified, // Be cautious
);
```text

### 4. Start with Warn Mode

When rolling out policy enforcement:

```rust
// Phase 1: Deploy with warn mode
let enforcer = PolicyEnforcer::warn_only();

// Phase 2: Review logs and fix violations

// Phase 3: Enable strict mode
let enforcer = PolicyEnforcer::strict();
```text

---

## Troubleshooting

### Error: "Capability not permitted"

```text
🚫 POLICY VIOLATION: Capability violations detected: FilesystemWrite
```text

**Solution**: Grant the capability in the manifest:

```rust
manifest.add_capability(
    Capability::FilesystemWrite,
    "Needs to save generated files",
);
manifest.save(&extensions_dir)?;
```text

### Warning: "Using deprecated mode"

```text
⚠️  POLICY WARNING: Capability violations detected: ProcessSpawn
   Enforcement mode is 'Warn' - operation allowed but flagged
```text

**Solution**: This is expected in Warn mode. Review the violation and either:
1. Grant the capability if legitimate
2. Fix the extension to not require it
3. Switch to Strict mode once ready

### Extension won't activate

**Check**:
1. Does a manifest exist?
2. Does it have required capabilities?
3. Is enforcement mode correct?

```rust
// Debug manifest loading
match ExtensionManifest::load(&extensions_dir, ext_id) {
    Ok(m) => println!("✅ Manifest loaded: {:?}", m),
    Err(e) => println!("❌ Manifest error: {}", e),
}
```text

---

## Migration Guide

### For Existing Extensions

1. **Create manifest**:

   ```rust
   let manifest = ExtensionManifest::new(ext_id, TrustLevel::UserTrusted);
```text

2. **Add capabilities based on usage**:

   ```rust
   // If extension reads files
   manifest.add_capability(Capability::FilesystemRead, "...");

   // If extension makes network calls
   manifest.add_capability(Capability::NetworkOutbound, "...");
```text

3. **Save manifest**:

   ```rust
   manifest.save(&extensions_dir)?;
```text

4. **Test in Warn mode**:

   ```rust
   let enforcer = PolicyEnforcer::warn_only();
```text

5. **Review logs for violations**

6. **Fix violations or grant capabilities**

7. **Switch to Strict mode**

---

## See Also

- [Full Upgrade Guide](../guides/VSC_CLI_NEXT_LEVEL_UPGRADE.md)
- [Phase 1 Completion Report](./VSC_CLI_UPGRADE_PHASE1_COMPLETE.md)
- [Capability Reference](https://docs.fusion-lang.org/policy/capabilities)

---

**Version**: 1.0
**Last Updated**: 2025-12-13
**Status**: Phase 1 Complete ✅