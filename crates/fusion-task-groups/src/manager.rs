// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! Task group management implementation
//!
//! Extensions to core task group types with management functionality

use crate::{PendingStep, ProgressUpdate, Subtask, TaskGroup, TaskGroupStatus};
use anyhow::Result;
use std::path::PathBuf;

impl TaskGroup {
    /// Create a new task group
    pub fn new(goal: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            goal,
            summary: String::new(),
            subtasks: Vec::new(),
            edited_files: Vec::new(),
            status: TaskGroupStatus::Pending,
        }
    }

    /// Add a subtask to the group
    pub fn add_subtask(&mut self, description: String) -> &mut Subtask {
        let subtask = Subtask {
            id: uuid::Uuid::new_v4().to_string(),
            description,
            progress_updates: Vec::new(),
            pending_steps: Vec::new(),
            expanded: false,
        };
        self.subtasks.push(subtask);
        self.subtasks.last_mut().unwrap()
    }

    /// Mark a file as edited
    pub fn mark_file_edited(&mut self, path: PathBuf) {
        if !self.edited_files.contains(&path) {
            self.edited_files.push(path);
        }
    }

    /// Start the task group
    pub fn start(&mut self) {
        self.status = TaskGroupStatus::InProgress;
    }

    /// Complete the task group
    pub fn complete(&mut self) {
        self.status = TaskGroupStatus::Completed;
    }

    /// Mark as failed
    pub fn fail(&mut self) {
        self.status = TaskGroupStatus::Failed;
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.subtasks.is_empty() {
            return 0.0;
        }

        let completed = self.subtasks.iter().filter(|st| st.is_complete()).count();

        (completed as f32 / self.subtasks.len() as f32) * 100.0
    }
}

impl Subtask {
    /// Add a progress update
    pub fn add_progress(&mut self, action: String, details: Option<String>) {
        self.progress_updates.push(ProgressUpdate {
            action,
            details,
            timestamp: chrono::Utc::now(),
        });
    }

    /// Add a pending step
    pub fn add_pending_step(&mut self, step: PendingStep) {
        self.pending_steps.push(step);
    }

    /// Check if subtask is complete
    pub fn is_complete(&self) -> bool {
        self.pending_steps.is_empty() && !self.progress_updates.is_empty()
    }

    /// Remove a pending step
    pub fn remove_pending_step(&mut self, index: usize) {
        if index < self.pending_steps.len() {
            self.pending_steps.remove(index);
        }
    }

    /// Toggle expanded state
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl PendingStep {
    /// Get a display string for the step
    pub fn display(&self) -> String {
        match self {
            Self::BrowserSetup { url } => format!("Browser: Navigate to {}", url),
            Self::TerminalCommand {
                command,
                needs_approval,
            } => {
                if *needs_approval {
                    format!("Terminal (needs approval): {}", command)
                } else {
                    format!("Terminal: {}", command)
                }
            }
            Self::FileEdit { path, needs_review } => {
                if *needs_review {
                    format!("Edit (needs review): {}", path.display())
                } else {
                    format!("Edit: {}", path.display())
                }
            }
        }
    }

    /// Check if step requires user approval
    pub fn requires_approval(&self) -> bool {
        match self {
            Self::BrowserSetup { .. } => true,
            Self::TerminalCommand { needs_approval, .. } => *needs_approval,
            Self::FileEdit { needs_review, .. } => *needs_review,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_group_creation() {
        let mut group = TaskGroup::new("Test Goal".to_string());
        assert_eq!(group.status, TaskGroupStatus::Pending);
        assert_eq!(group.subtasks.len(), 0);

        group.add_subtask("Subtask 1".to_string());
        assert_eq!(group.subtasks.len(), 1);
    }

    #[test]
    fn test_completion_percentage() {
        let mut group = TaskGroup::new("Test".to_string());
        let subtask1 = group.add_subtask("Task 1".to_string());
        subtask1.add_progress("Done".to_string(), None);

        group.add_subtask("Task 2".to_string());

        // 1 out of 2 complete = 50%
        assert_eq!(group.completion_percentage(), 50.0);
    }

    #[test]
    fn test_pending_step_display() {
        let step = PendingStep::TerminalCommand {
            command: "cargo build".to_string(),
            needs_approval: true,
        };

        assert!(step.display().contains("needs approval"));
        assert!(step.requires_approval());
    }
}
