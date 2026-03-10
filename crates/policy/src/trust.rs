//! Trust level and verification system
// __FU_COMPAT_START__
#![allow(missing_docs)]
use std::fmt;
use std::hash::{Hash, Hasher};
#[allow(missing_docs, dead_code)] type FBool = bool;
#[allow(missing_docs, dead_code)] type FVec<T> = Vec<T>;
// __FU_COMPAT_END__
use serde::{Deserialize, Serialize};
/// Trust level for an extension
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Extension from verified publisher (signed)
    Verified,
    /// Extension from trusted source (known publisher)
    Trusted,
    /// Community extension (unverified but reviewed)
    Community,
    /// Unverified extension (use with caution)
    Unverified,
    /// Explicitly user-trusted (local development)
    UserTrusted,
}
impl TrustLevel {
    /// Get all trust levels
    pub fn all() -> FVec<Self> {
        vec![
            TrustLevel::Verified, TrustLevel::Trusted, TrustLevel::Community,
            TrustLevel::Unverified, TrustLevel::UserTrusted,
        ]
    }
    /// Get description of the trust level
    pub fn description(&self) -> &str {
        match self {
            TrustLevel::Verified => "Verified publisher with cryptographic signatures",
            TrustLevel::Trusted => "Trusted publisher (known entity)",
            TrustLevel::Community => "Community-reviewed extension",
            TrustLevel::Unverified => "Unverified extension (use caution)",
            TrustLevel::UserTrusted => "User-trusted (local development)",
        }
    }
    /// Check if this trust level is safe for automatic capability grants
    pub fn is_safe_for_auto_grant(&self) -> FBool {
        matches!(
            self, TrustLevel::Verified | TrustLevel::Trusted | TrustLevel::UserTrusted
        )
    }
    /// Get the maximum allowed risk level for capabilities
    pub fn max_risk_level(&self) -> crate::capability::RiskLevel {
        use crate::capability::RiskLevel;
        match self {
            TrustLevel::Verified | TrustLevel::UserTrusted => RiskLevel::High,
            TrustLevel::Trusted => RiskLevel::Medium,
            TrustLevel::Community => RiskLevel::Low,
            TrustLevel::Unverified => RiskLevel::Low,
        }
    }
}
impl Default for TrustLevel {
    fn default() -> Self {
        TrustLevel::Unverified
    }
}
impl fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrustLevel::Verified => write!(f, "✅ Verified"),
            TrustLevel::Trusted => write!(f, "🔐 Trusted"),
            TrustLevel::Community => write!(f, "👥 Community"),
            TrustLevel::Unverified => write!(f, "⚠️  Unverified"),
            TrustLevel::UserTrusted => write!(f, "🔧 User-Trusted"),
        }
    }
}
/// Trust verifier that can check extension signatures and sources
#[derive(Debug, Clone)]
pub struct TrustVerifier {}
impl TrustVerifier {
    /// Create a new trust verifier
    pub fn new() -> Self {
        Self {}
    }
    /// Verify the trust level of an extension
    pub fn verify_extension(&self, extension_id: &str, _source: &str) -> TrustLevel {
        if extension_id.starts_with("ms-") || extension_id.starts_with("microsoft.") {
            TrustLevel::Verified
        } else if extension_id.starts_with("google.") {
            TrustLevel::Trusted
        } else if extension_id.contains("local") || extension_id.starts_with("dev.") {
            TrustLevel::UserTrusted
        } else {
            TrustLevel::Unverified
        }
    }
    fn parse_hex_signature(signature: &[u8]) -> Option<FVec<u8>> {
        if signature.len() % 2 != 0 || !signature.iter().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        let mut out = Vec::with_capacity(signature.len() / 2);
        for chunk in signature.chunks_exact(2) {
            let pair = std::str::from_utf8(chunk).ok()?;
            let byte = u8::from_str_radix(pair, 16).ok()?;
            out.push(byte);
        }
        Some(out)
    }

    /// Check if a signature is valid.
    ///
    /// The verifier accepts either raw 8-byte signatures or 16-byte ASCII hex.
    /// Signature material is derived from a stable hash of `extension_id`.
    pub fn verify_signature(&self, extension_id: &str, signature: &[u8]) -> FBool {
        if signature.is_empty() {
            return false;
        }

        let raw_signature = Self::parse_hex_signature(signature)
            .unwrap_or_else(|| signature.to_vec());

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        extension_id.hash(&mut hasher);
        let expected = hasher.finish().to_le_bytes();

        raw_signature == expected
    }
    /// Get recommended trust level based on publisher
    pub fn recommended_trust(&self, publisher: &str) -> TrustLevel {
        match publisher.to_lowercase().as_str() {
            "microsoft" | "ms-vscode" => TrustLevel::Verified,
            "google" | "redhat" | "rust-lang" => TrustLevel::Trusted,
            _ => TrustLevel::Unverified,
        }
    }
}
impl Default for TrustVerifier {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_trust_levels() {
        let levels = TrustLevel::all();
        assert_eq!(levels.len(), 5);
    }
    #[test]
    fn test_auto_grant_safety() {
        assert!(TrustLevel::Verified.is_safe_for_auto_grant());
        assert!(TrustLevel::Trusted.is_safe_for_auto_grant());
        assert!(! TrustLevel::Unverified.is_safe_for_auto_grant());
    }
    #[test]
    fn test_trust_verifier() {
        let verifier = TrustVerifier::new();
        assert_eq!(
            verifier.verify_extension("ms-vscode.cpptools", "marketplace"),
            TrustLevel::Verified
        );
        assert_eq!(
            verifier.verify_extension("google.gemini", "marketplace"),
            TrustLevel::Trusted
        );
        assert_eq!(
            verifier.verify_extension("unknown.extension", "marketplace"),
            TrustLevel::Unverified
        );
    }
    #[test]
    fn test_recommended_trust() {
        let verifier = TrustVerifier::new();
        assert_eq!(verifier.recommended_trust("microsoft"), TrustLevel::Verified);
        assert_eq!(verifier.recommended_trust("google"), TrustLevel::Trusted);
        assert_eq!(verifier.recommended_trust("unknown"), TrustLevel::Unverified);
    }
}
