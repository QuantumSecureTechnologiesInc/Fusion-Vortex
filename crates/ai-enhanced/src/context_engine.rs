//! Context engine for managing code context

use super::interactive_agent::ContextType;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ContextEngine {
    contexts: HashMap<String, ContextEntry>,
}

#[derive(Debug, Clone)]
struct ContextEntry {
    _context_type: String,
    content: String,
}

impl ContextEngine {
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    pub async fn add_context(&mut self, context_type: ContextType, content: String) -> Result<()> {
        let key = uuid::Uuid::new_v4().to_string();
        self.contexts.insert(
            key,
            ContextEntry {
                _context_type: format!("{:?}", context_type),
                content,
            },
        );
        Ok(())
    }

    pub fn get_all_context(&self) -> String {
        self.contexts
            .values()
            .map(|e| e.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    pub fn clear(&mut self) {
        self.contexts.clear();
    }
}
