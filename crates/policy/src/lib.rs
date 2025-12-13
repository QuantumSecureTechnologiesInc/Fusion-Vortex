//! Fusion Policy Engine
//!
//! Provides capability-based security enforcement for extensions and MCP tools.
//! This is the critical security layer that gates all privileged operations.

pub mod capability;
pub mod enforcement;
pub mod manifest;
pub mod trust;

pub use capability::Capability;
pub use enforcement::{EnforcementMode, PolicyEnforcer};
pub use manifest::{CapabilityManifest, ExtensionManifest};
pub use trust::{TrustLevel, TrustVerifier};

/// Policy engine version
pub const POLICY_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_version() {
        assert!(!POLICY_VERSION.is_empty());
    }
}
