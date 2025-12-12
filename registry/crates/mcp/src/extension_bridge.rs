use fusion_vscode_runtime::ExtensionHost;
use std::sync::Arc;

/// Bridge between MCP and VS Code extensions
pub struct ExtensionMcpBridge {
    host: Arc<ExtensionHost>,
}

impl ExtensionMcpBridge {
    /// Create a new bridge with the given extension host
    pub fn new(host: Arc<ExtensionHost>) -> Self {
        Self { host }
    }
}
