// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Fusion Agent Core
//!
//! Core agent orchestration logic for Planning/Fast modes,
//! continuous context, and agent session management.

pub mod conversation;
pub mod modes;
pub mod secure_mode;
pub mod session;

pub use conversation::{ConversationContext, Message, UserInterrupt};
pub use modes::{AgentMode, AgentModeType};
pub use secure_mode::SecureMode;
pub use session::{AgentSession, SessionMetadata};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::conversation::*;
    pub use crate::modes::*;
    pub use crate::secure_mode::*;
    pub use crate::session::*;
}
