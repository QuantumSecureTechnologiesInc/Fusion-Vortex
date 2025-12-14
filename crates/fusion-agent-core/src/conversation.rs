//! Conversation context for continuous messaging
//!
//! Supports sending messages to agent during execution,
//! user interrupts, and context injection

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub messages: Vec<Message>,
    pub interrupts: Vec<UserInterrupt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInterrupt {
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub handled: bool,
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            interrupts: Vec::new(),
        }
    }

    pub fn add_message(&mut self, role: &str, content: String) {
        self.messages.push(Message {
            role: role.to_string(),
            content,
            timestamp: Utc::now(),
        });
    }

    pub fn add_user_message(&mut self, content: String) {
        self.add_message("user", content);
    }

    pub fn add_assistant_message(&mut self, content: String) {
        self.add_message("assistant", content);
    }

    /// Add an interrupt (message sent while agent is working)
    pub fn add_interrupt(&mut self, message: String) {
        self.interrupts.push(UserInterrupt {
            message,
            timestamp: Utc::now(),
            handled: false,
        });
    }

    /// Get unhandled interrupts
    pub fn pending_interrupts(&self) -> Vec<&UserInterrupt> {
        self.interrupts.iter().filter(|i| !i.handled).collect()
    }

    /// Mark interrupt as handled
    pub fn mark_interrupt_handled(&mut self, index: usize) {
        if let Some(interrupt) = self.interrupts.get_mut(index) {
            interrupt.handled = true;
        }
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get last N messages
    pub fn last_messages(&self, count: usize) -> &[Message] {
        let start = self.messages.len().saturating_sub(count);
        &self.messages[start..]
    }
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_messages() {
        let mut ctx = ConversationContext::new();
        ctx.add_user_message("Hello".to_string());
        ctx.add_assistant_message("Hi there".to_string());

        assert_eq!(ctx.message_count(), 2);
        assert_eq!(ctx.messages[0].role, "user");
        assert_eq!(ctx.messages[1].role, "assistant");
    }

    #[test]
    fn test_interrupts() {
        let mut ctx = ConversationContext::new();
        ctx.add_interrupt("Stop!".to_string());

        assert_eq!(ctx.pending_interrupts().len(), 1);

        ctx.mark_interrupt_handled(0);
        assert_eq!(ctx.pending_interrupts().len(), 0);
    }

    #[test]
    fn test_last_messages() {
        let mut ctx = ConversationContext::new();
        ctx.add_user_message("Msg 1".to_string());
        ctx.add_user_message("Msg 2".to_string());
        ctx.add_user_message("Msg 3".to_string());

        let last = ctx.last_messages(2);
        assert_eq!(last.len(), 2);
        assert_eq!(last[0].content, "Msg 2");
        assert_eq!(last[1].content, "Msg 3");
    }
}
