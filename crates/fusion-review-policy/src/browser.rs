// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Integration helpers for browser agent policy enforcement

use crate::{BrowserPolicy, JavascriptPolicy};

impl BrowserPolicy {
    /// Create a safe default browser policy
    pub fn safe_default() -> Self {
        Self {
            javascript_execution: JavascriptPolicy::RequestReview,
            url_allowlist: vec![
                "github.com".to_string(),
                "docs.rs".to_string(),
                "crates.io".to_string(),
            ],
            url_denylist: Vec::new(),
        }
    }

    /// Create a locked-down policy (deny all except specific sites)
    pub fn locked_down(allowed_sites: Vec<String>) -> Self {
        Self {
            javascript_execution: JavascriptPolicy::RequestReview,
            url_allowlist: allowed_sites,
            url_denylist: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_default() {
        let policy = BrowserPolicy::safe_default();
        assert!(policy.can_navigate("https://github.com/user/repo"));
        assert!(policy.can_navigate("https://docs.rs/tokio"));
        assert!(!policy.can_execute_javascript());
    }

    #[test]
    fn test_locked_down() {
        let policy = BrowserPolicy::locked_down(vec!["example.com".to_string()]);
        assert!(policy.can_navigate("https://example.com/page"));
        assert!(!policy.can_navigate("https://other.com"));
    }
}
