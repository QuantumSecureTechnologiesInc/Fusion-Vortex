use anyhow::Result;
use async_trait::async_trait;
use fusion_policy::Capability;
use fusion_vscode_runtime::compat::CompatibilityLevel;
use serde_json::Value;
use std::fmt;

/// Trait for handling MCP tool execution requests
#[async_trait]
pub trait McpHandler: Send + Sync {
    /// Execute the handler with the given arguments
    async fn handle(&self, arguments: Option<Value>) -> Result<crate::protocol::CallToolResult>;
}

/// Represents a distinct facet/action of a tool
pub struct McpToolFacet {
    pub name: String,
    pub description: String,
    pub handler: Box<dyn McpHandler>,
}

impl fmt::Debug for McpToolFacet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("McpToolFacet")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

/// Rich tool representation with facets (MCP v1.0 compliant)
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub facets: Vec<McpToolFacet>,
    /// Required capabilities for this tool (v1.0)
    pub capabilities: Vec<Capability>,
    /// Compatibility level (v1.0)
    pub compatibility: CompatibilityLevel,
}

impl McpTool {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            facets: Vec::new(),
            capabilities: Vec::new(),
            compatibility: CompatibilityLevel::Full,
        }
    }

    /// Create tool with explicit capabilities and compatibility
    pub fn with_requirements(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
        capabilities: Vec<Capability>,
        compatibility: CompatibilityLevel,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            facets: Vec::new(),
            capabilities,
            compatibility,
        }
    }

    pub fn add_facet(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        handler: Box<dyn McpHandler>,
    ) {
        self.facets.push(McpToolFacet {
            name: name.into(),
            description: description.into(),
            handler,
        });
    }

    /// Flatten this tool into a list of specific tool names (e.g. tool.facet)
    pub fn flattened_names(&self) -> Vec<String> {
        if self.facets.is_empty() {
            vec![self.name.clone()]
        } else {
            self.facets
                .iter()
                .map(|f| format!("{}.{}", self.name, f.name))
                .collect()
        }
    }
}
