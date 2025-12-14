use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// First-run welcome experience
pub struct WelcomeScreen;

impl WelcomeScreen {
    /// Check if this is the first run
    pub fn is_first_run() -> bool {
        let config_file = Self::config_marker();
        !config_file.exists()
    }

    /// Get the config marker file path
    fn config_marker() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".fusion")
            .join(".initialized")
    }

    /// Mark first run as complete
    pub fn mark_initialized() -> Result<()> {
        let marker = Self::config_marker();
        if let Some(parent) = marker.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(marker, "initialized")?;
        Ok(())
    }

    /// Display welcome message and tutorial
    pub fn display() {
        println!("\n{}", "=".repeat(60));
        println!(
            "  🚀 Welcome to Fusion VSC CLI v{}",
            env!("CARGO_PKG_VERSION")
        );
        println!("{}\n", "=".repeat(60));

        println!("Fusion is a secure, policy-enforced CLI for running VS Code");
        println!("extensions headlessly with MCP tooling & agent orchestration.\n");

        println!("📚 Quick Start Guide:\n");

        Self::show_quick_start();

        println!("\n💡 Key Features:\n");
        Self::show_features();

        println!("\n🔗 Helpful Resources:\n");
        Self::show_resources();

        println!("\n{}", "-".repeat(60));
        println!("  💬 Need help? Run:  fusion --help");
        println!("{}\n", "-".repeat(60));
    }

    fn show_quick_start() {
        println!("  1️⃣  Check your installation:");
        println!("      fusion --version\n");

        println!("  2️⃣  Install an extension:");
        println!("      fusion extensions install google.gemini-code-assist\n");

        println!("  3️⃣  Grant capabilities (permissions):");
        println!("      fusion policy grant google.gemini-code-assist FilesystemRead\n");

        println!("  4️⃣  Start MCP server:");
        println!("      fusion mcp serve --port 3000\n");

        println!("  5️⃣  Check extension compatibility:");
        println!("      fusion extensions doctor google.gemini-code-assist");
    }

    fn show_features() {
        println!("  ✓ Capability-Based Security (explicit permissions)");
        println!("  ✓ MCP v1.0 Protocol (normative, streaming)");
        println!("  ✓ Agent Orchestration (deterministic, auditable plans)");
        println!("  ✓ LSP Integration (language intelligence as resources)");
        println!("  ✓ Headless Extensions (CLI automation)");
    }

    fn show_resources() {
        println!("  📖 FAQ & Getting Started:  docs/FAQ.md");
        println!("  🔧 Troubleshooting Guide:   docs/TROUBLESHOOTING.md");
        println!("  📋 Command Reference:      fusion --help");
        println!("  🧪 Test Your Setup:        fusion extensions doctor");
    }

    /// Show interactive tips based on command
    pub fn show_tip(command: &str) {
        match command {
            "mcp" => {
                println!("\n💡 Tip: MCP server exposes tools for AI agents.");
                println!("   Connect with any MCP-compatible client on the specified port.");
            }
            "policy" => {
                println!("\n💡 Tip: Start with 'warn' mode during testing:");
                println!("   fusion policy mode warn");
                println!("   Then switch to 'strict' for production.");
            }
            "extensions" => {
                println!("\n💡 Tip: Check compatibility before installing:");
                println!("   fusion extensions doctor <extension-id>");
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_marker_path() {
        let marker = WelcomeScreen::config_marker();
        assert!(marker.to_string_lossy().contains(".fusion"));
        assert!(marker.to_string_lossy().contains(".initialized"));
    }
}
