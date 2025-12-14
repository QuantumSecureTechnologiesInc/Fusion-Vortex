// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Policy enforcement implementation
//!
//! Validates terminal commands, browser actions, and artifact requests
//! against configured review policies

use crate::{
    ArtifactPolicy, BrowserPolicy, JavascriptPolicy, ReviewPolicy, TerminalExecutionMode,
    TerminalPolicy,
};
use anyhow::Result;

impl ReviewPolicy {
    /// Create a default lenient policy
    pub fn lenient() -> Self {
        Self {
            artifact: ArtifactPolicy::AlwaysProceed,
            terminal: TerminalPolicy {
                mode: TerminalExecutionMode::AlwaysProceed,
                allow_list: Vec::new(),
                deny_list: Vec::new(),
            },
            browser: BrowserPolicy {
                javascript_execution: JavascriptPolicy::AlwaysProceed,
                url_allowlist: Vec::new(),
                url_denylist: Vec::new(),
            },
        }
    }

    /// Create a strict review-required policy
    pub fn strict() -> Self {
        Self {
            artifact: ArtifactPolicy::RequestReview,
            terminal: TerminalPolicy {
                mode: TerminalExecutionMode::RequestReview,
                allow_list: Vec::new(),
                deny_list: vec!["rm".to_string(), "sudo".to_string()],
            },
            browser: BrowserPolicy {
                javascript_execution: JavascriptPolicy::RequestReview,
                url_allowlist: Vec::new(),
                url_denylist: Vec::new(),
            },
        }
    }
}

impl TerminalPolicy {
    /// Check if a command is allowed to auto-execute
    pub fn can_auto_execute(&self, command: &str) -> bool {
        // If mode is always proceed, check allow/deny lists
        if self.mode == TerminalExecutionMode::AlwaysProceed {
            return self.is_command_allowed(command);
        }

        // If mode is request review, only auto-execute if in allow list
        if !self.allow_list.is_empty() {
            return self.is_in_allow_list(command);
        }

        false
    }

    /// Check if command is explicitly allowed
    fn is_command_allowed(&self, command: &str) -> bool {
        // Check deny list first
        if self.is_denied(command) {
            return false;
        }

        // If allow list is empty, allow all (except denied)
        if self.allow_list.is_empty() {
            return true;
        }

        // Check allow list
        self.is_in_allow_list(command)
    }

    fn is_in_allow_list(&self, command: &str) -> bool {
        self.allow_list
            .iter()
            .any(|pattern| command.starts_with(pattern) || command.contains(pattern))
    }

    fn is_denied(&self, command: &str) -> bool {
        self.deny_list
            .iter()
            .any(|pattern| command.starts_with(pattern) || command.contains(pattern))
    }

    /// Get a safe subset of common read-only commands
    pub fn safe_readonly_commands() -> Vec<String> {
        vec![
            "ls".to_string(),
            "cat".to_string(),
            "grep".to_string(),
            "find".to_string(),
            "git status".to_string(),
            "git diff".to_string(),
            "cargo check".to_string(),
        ]
    }
}

impl BrowserPolicy {
    /// Check if URL navigation is allowed
    pub fn can_navigate(&self, url: &str) -> bool {
        // Check denylist first
        if self.is_url_denied(url) {
            return false;
        }

        // If allowlist is empty, allow all (except denied)
        if self.url_allowlist.is_empty() {
            return true;
        }

        // Check allowlist
        self.is_url_allowed(url)
    }

    fn is_url_allowed(&self, url: &str) -> bool {
        self.url_allowlist
            .iter()
            .any(|pattern| url.contains(pattern))
    }

    fn is_url_denied(&self, url: &str) -> bool {
        self.url_denylist
            .iter()
            .any(|pattern| url.contains(pattern))
    }

    /// Check if JavaScript execution is allowed
    pub fn can_execute_javascript(&self) -> bool {
        matches!(self.javascript_execution, JavascriptPolicy::AlwaysProceed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_policy_allow_list() {
        let policy = TerminalPolicy {
            mode: TerminalExecutionMode::RequestReview,
            allow_list: vec!["git".to_string(), "cargo check".to_string()],
            deny_list: Vec::new(),
        };

        assert!(policy.can_auto_execute("git status"));
        assert!(policy.can_auto_execute("cargo check"));
        assert!(!policy.can_auto_execute("rm -rf /"));
    }

    #[test]
    fn test_terminal_policy_deny_list() {
        let policy = TerminalPolicy {
            mode: TerminalExecutionMode::AlwaysProceed,
            allow_list: Vec::new(),
            deny_list: vec!["rm".to_string(), "sudo".to_string()],
        };

        assert!(policy.can_auto_execute("git status"));
        assert!(!policy.can_auto_execute("rm file.txt"));
        assert!(!policy.can_auto_execute("sudo apt install"));
    }

    #[test]
    fn test_browser_policy_url_filtering() {
        let mut policy = BrowserPolicy {
            javascript_execution: JavascriptPolicy::RequestReview,
            url_allowlist: vec!["github.com".to_string()],
            url_denylist: vec!["evil.com".to_string()],
        };

        assert!(policy.can_navigate("https://github.com/repo"));
        assert!(!policy.can_navigate("https://evil.com"));
        assert!(!policy.can_navigate("https://random.com")); // Not in allowlist

        // Empty allowlist = allow all
        policy.url_allowlist.clear();
        assert!(policy.can_navigate("https://random.com"));
    }

    #[test]
    fn test_strict_policy() {
        let policy = ReviewPolicy::strict();
        assert_eq!(policy.artifact, ArtifactPolicy::RequestReview);
        assert_eq!(policy.terminal.mode, TerminalExecutionMode::RequestReview);
    }
}
