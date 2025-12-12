//! TUI Dashboard for real-time status display

use crate::state::{BuildStatus, SharedState};
use std::time::Duration;

/// TUI Dashboard for displaying build status
pub struct Dashboard {
    state: SharedState,
}

impl Dashboard {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    /// Runs the dashboard render loop
    pub fn run_loop(&self, refresh_ms: u64) {
        let mut last_status = String::new();

        loop {
            std::thread::sleep(Duration::from_millis(refresh_ms));

            let (status_line, logs) = {
                let state = self.state.read();
                (state.status.to_string(), state.stdout_buffer.clone())
            };

            if status_line != last_status {
                println!("\x1b[2K\r[Fusion] {}", status_line);
                last_status = status_line;
            }

            for log in &logs {
                println!(" > {}", log);
            }

            if !logs.is_empty() {
                self.state.write().stdout_buffer.clear();
            }

            // Check for completion
            let is_complete = self.state.read().status.is_complete();
            if is_complete {
                break;
            }
        }
    }

    /// Renders a single frame (for testing)
    pub fn render_frame(&self) -> String {
        let state = self.state.read();
        format!("[Fusion] {}", state.status)
    }
}
