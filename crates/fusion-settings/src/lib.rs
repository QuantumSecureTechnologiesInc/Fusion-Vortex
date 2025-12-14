// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Fusion Settings
//!
//! Hierarchical settings management with precedence:
//! Enterprise > CLI > Local > Project > User

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod loader;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    // Core settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSettings {
    pub allow: Vec<String>,
    pub ask: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSettings {
    pub default_mode: String,
}

pub struct SettingsLoader {
    pub(crate) enterprise: Option<Settings>,
    pub(crate) cli_args: Option<Settings>,
    pub(crate) local: Option<Settings>,
    pub(crate) project: Option<Settings>,
    pub(crate) user: Option<Settings>,
}

impl SettingsLoader {
    pub fn new() -> Self {
        Self {
            enterprise: None,
            cli_args: None,
            local: None,
            project: None,
            user: None,
        }
    }
}

impl Default for SettingsLoader {
    fn default() -> Self {
        Self::new()
    }
}
