//! Fusion Task Groups
//!
//! Task group management with subtasks, progress tracking,
//! and pending step handling.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod manager;
pub mod ui;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGroup {
    pub id: String,
    pub goal: String,
    pub summary: String,
    pub subtasks: Vec<Subtask>,
    pub edited_files: Vec<PathBuf>,
    pub status: TaskGroupStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtask {
    pub id: String,
    pub description: String,
    pub progress_updates: Vec<ProgressUpdate>,
    pub pending_steps: Vec<PendingStep>,
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub action: String,
    pub details: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PendingStep {
    BrowserSetup {
        url: String,
    },
    TerminalCommand {
        command: String,
        needs_approval: bool,
    },
    FileEdit {
        path: PathBuf,
        needs_review: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskGroupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

// Re-export UI rendering
pub use ui::render_task_group_ui;
