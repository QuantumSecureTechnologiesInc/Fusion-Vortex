// Copyright (c) 2024 QuantumSecure Technologies Inc / Fusion Programming Language Team
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Fusion VSC CLI Coder

//! UI rendering helpers for task groups
//!
//! Provides display formatting for TUI integration

use crate::{PendingStep, Subtask, TaskGroup};

impl TaskGroup {
    /// Get a formatted header for display
    pub fn header(&self) -> String {
        format!(
            "{} - {:.0}% complete",
            self.goal,
            self.completion_percentage()
        )
    }

    /// Get file pills for display (clickable file names)
    pub fn file_pills(&self) -> Vec<String> {
        self.edited_files
            .iter()
            .map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            })
            .collect()
    }

    /// Get summary text for display
    pub fn summary_text(&self) -> &str {
        if self.summary.is_empty() {
            "No summary available"
        } else {
            &self.summary
        }
    }

    /// Format status badge
    pub fn status_badge(&self) -> (&'static str, &'static str) {
        match self.status {
            crate::TaskGroupStatus::Pending => ("PENDING", "yellow"),
            crate::TaskGroupStatus::InProgress => ("IN PROGRESS", "cyan"),
            crate::TaskGroupStatus::Completed => ("COMPLETE", "green"),
            crate::TaskGroupStatus::Failed => ("FAILED", "red"),
        }
    }
}

impl Subtask {
    /// Get a formatted display line
    pub fn display_line(&self) -> String {
        let status = if self.is_complete() { "✓" } else { "○" };
        format!("{} {}", status, self.description)
    }

    /// Get progress updates as formatted strings
    pub fn formatted_progress(&self) -> Vec<String> {
        self.progress_updates
            .iter()
            .map(|pu| {
                let time = pu.timestamp.format("%H:%M:%S");
                match &pu.details {
                    Some(details) => format!("[{}] {} - {}", time, pu.action, details),
                    None => format!("[{}] {}", time, pu.action),
                }
            })
            .collect()
    }

    /// Get pending steps count
    pub fn pending_count(&self) -> usize {
        self.pending_steps.len()
    }
}

/// Helper to render task group for TUI
pub fn render_task_group_ui(group: &TaskGroup) -> Vec<String> {
    let mut lines = Vec::new();

    // Header
    lines.push(group.header());
    lines.push(String::new());

    // Summary
    lines.push(format!("Summary: {}", group.summary_text()));
    lines.push(String::new());

    // Edited files
    if !group.edited_files.is_empty() {
        lines.push("Edited Files:".to_string());
        for file in group.file_pills() {
            lines.push(format!("  [{}]", file));
        }
        lines.push(String::new());
    }

    // Subtasks
    lines.push("Subtasks:".to_string());
    for subtask in &group.subtasks {
        lines.push(format!("  {}", subtask.display_line()));

        if subtask.expanded {
            for progress in subtask.formatted_progress() {
                lines.push(format!("    {}", progress));
            }
        }

        if !subtask.pending_steps.is_empty() {
            lines.push(format!("    Pending: {} steps", subtask.pending_count()));
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_group_header() {
        let group = TaskGroup::new("Build Feature".to_string());
        assert!(group.header().contains("Build Feature"));
        assert!(group.header().contains("%"));
    }

    #[test]
    fn test_file_pills() {
        let mut group = TaskGroup::new("Test".to_string());
        group.mark_file_edited(std::path::PathBuf::from("/path/to/file.rs"));

        let pills = group.file_pills();
        assert_eq!(pills.len(), 1);
        assert_eq!(pills[0], "file.rs");
    }

    #[test]
    fn test_render_ui() {
        let mut group = TaskGroup::new("Test Goal".to_string());
        group.summary = "Test summary".to_string();
        group.add_subtask("Task 1".to_string());

        let lines = render_task_group_ui(&group);
        assert!(lines.iter().any(|l| l.contains("Test Goal")));
        assert!(lines.iter().any(|l| l.contains("Test summary")));
    }
}
