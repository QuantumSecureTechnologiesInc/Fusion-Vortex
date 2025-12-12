//! Security Auditor - Shift-Left Security Scanning

use semver::{Version, VersionReq};
use std::collections::HashMap;

/// A security vulnerability
#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub id: String,
    pub title: String,
    pub affected_versions: String,
    pub cvss_score: Option<f32>,
    pub severity: Severity,
}

/// Vulnerability severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// The Security Auditor
pub struct Auditor {
    db: HashMap<String, Vec<Vulnerability>>,
}

impl Auditor {
    pub fn new() -> Self {
        let mut db = HashMap::new();
        db.insert(
            "openssl".to_string(),
            vec![Vulnerability {
                id: "RUSTSEC-2023-0072".to_string(),
                title: "Buffer Overflow in legacy OpenSSL bindings".to_string(),
                affected_versions: "<0.10.40".to_string(),
                cvss_score: Some(7.5),
                severity: Severity::High,
            }],
        );
        db.insert("serde".to_string(), vec![]);
        db.insert("log".to_string(), vec![]);
        Self { db }
    }

    pub fn check(&self, name: &str, version: &str) -> Result<(), String> {
        if let Some(vulns) = self.db.get(name) {
            for vuln in vulns {
                if self.is_version_affected(version, &vuln.affected_versions) {
                    return Err(format!("{}: {}", vuln.id, vuln.title));
                }
            }
        }
        Ok(())
    }

    fn is_version_affected(&self, version: &str, req: &str) -> bool {
        if let Ok(ver) = Version::parse(version) {
            if let Ok(req) = VersionReq::parse(req) {
                return req.matches(&ver);
            }
        }
        if req.starts_with('<') {
            return version < &req[1..];
        }
        false
    }
}

impl Default for Auditor {
    fn default() -> Self {
        Self::new()
    }
}
