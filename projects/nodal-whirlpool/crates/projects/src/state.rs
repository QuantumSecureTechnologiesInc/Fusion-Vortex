use crate::{CodeChange, ConversationMessage, Project};
use serde::{Deserialize, Serialize};

/// Complete project state for export/import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub project: Project,
    pub sessions: Vec<SessionInfo>,
    pub conversations: Vec<ConversationMessage>,
    pub changes: Vec<CodeChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: i64,
    pub started_at: String,
    pub ended_at: Option<String>,
}
