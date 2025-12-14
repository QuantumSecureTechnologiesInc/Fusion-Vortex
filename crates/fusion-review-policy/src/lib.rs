// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Fusion Review Policy
//!
//! Review policy enforcement for artifacts, terminal commands,
//! and browser actions.

use serde::{Deserialize, Serialize};

pub mod browser;
pub mod enforcement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewPolicy {
    pub artifact: ArtifactPolicy,
    pub terminal: TerminalPolicy,
    pub browser: BrowserPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactPolicy {
    AlwaysProceed,
    RequestReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalPolicy {
    pub mode: TerminalExecutionMode,
    pub allow_list: Vec<String>,
    pub deny_list: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerminalExecutionMode {
    RequestReview,
    AlwaysProceed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserPolicy {
    pub javascript_execution: JavascriptPolicy,
    pub url_allowlist: Vec<String>,
    pub url_denylist: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JavascriptPolicy {
    RequestReview,
    AlwaysProceed,
}
