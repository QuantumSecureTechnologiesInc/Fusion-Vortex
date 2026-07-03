// The Chaos Vacuum Diagnostics Engine
// Rationale: Replaces raw, unreadable compiler backtraces with highly descriptive,
// conversational terminal reports that explain the thermodynamic "Why" of safety errors.

use crate::vortex::EventCollision;
use crate::vortex::PermissionState;

pub struct ChaosVacuumReporter {
    source_filename: String,
    source_code: String,
}

impl ChaosVacuumReporter {
    pub fn new(filename: &str, code: &str) -> Self {
        Self {
            source_filename: filename.to_string(),
            source_code: code.to_string(),
        }
    }

    /// Renders a gorgeous conversational terminal report showing exactly where
    /// the permission flow collided and how to resolve the conflict safely.
    pub fn publish_collision_report(&self, event: &EventCollision) {
        println!("\x1b[1;31mEntropic Collision Detected in {}\x1b[0m", self.source_filename);
        println!("\x1b[1;30m======================================================================\x1b[0m");
        println!("Variable flow \x1b[1;36m'{}'\x1b[0m suffered a permission stream intersection.", event.var_name);
        
        let state_description = match event.existing_state {
            PermissionState::Dissipated => "consumed or moved into a different execution scope",
            PermissionState::ExclusiveBorrowed => "exclusively borrowed by a mutable writer",
            PermissionState::SharedBorrowed(_count) => "borrowed immutably by active shared readers",
            PermissionState::Intact => "residing intact inside standard local scope",
        };

        println!("\n\x1b[1;33mAnalysis of Flow Collision:\x1b[0m");
        println!("  * At line space, the resource was already: \x1b[1;35m{}\x1b[0m.", state_description);
        println!("  * You attempted to access or borrow it here:");
        
        self.render_source_slice(event.collision_span.start, event.collision_span.end);
        
        println!("\x1b[1;32mRemediation Advice:\x1b[0m");
        match event.existing_state {
            PermissionState::Dissipated => {
                println!("    To repair this flow, allocate a fresh resource or structure your code to complete");
                println!("    the work before passing ownership to subsequent execution scopes.");
            }
            PermissionState::ExclusiveBorrowed | PermissionState::SharedBorrowed(_) => {
                println!("    Fusion's Vortex Engine strictly forbids conflicting read/write access.");
                println!("    Wrap the borrow blocks inside explicit scope boundaries using blocks '{{ ... }}'");
                println!("    to allow exclusive permission frames to exit before you access the resource again.");
            }
            PermissionState::Intact => {}
        }
        println!("\x1b[1;30m======================================================================\x1b[0m\n");
    }

    fn render_source_slice(&self, start: usize, end: usize) {
        let _lines: Vec<&str> = self.source_code.lines().collect();
        // Fallback bounds protection mapping across lines
        let safe_start = start.min(self.source_code.len());
        let safe_end = end.min(self.source_code.len());
        
        println!("\n    \x1b[1;34m| ");
        println!("    | \x1b[0m{}", &self.source_code[safe_start..safe_end]);
        println!("    \x1b[1;34m| \x1b[1;31m^^^^^ Collision Vector\x1b[0m\n");
    }
}