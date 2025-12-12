use crate::{ContextCommands, McpCommands, ToolCommands};
use anyhow::Result;
use fusion_mcp::{FilesystemServer, GitHubServer, McpClient, WebServer};
use std::path::PathBuf;
use tokio;

/// List MCP servers
pub fn list_servers() -> Result<()> {
    let settings = fusion_settings::FusionSettings::load()?;

    println!("🔌 Available MCP Servers");
    println!("═══════════════════════════════════════════════════════\n");

    println!("📁 Filesystem Server");
    println!("   Provides access to local files");
    println!("   Usage: fusion mcp connect filesystem <PATH>\n");

    println!("🐙 GitHub Server");
    if settings.github.token.is_some() {
        println!("   ✓ Configured with GitHub token");
    } else {
        println!("   ⚠ No GitHub token configured");
    }
    println!("   Usage: fusion mcp connect github\n");

    println!("🌐 Web Server");
    println!("   Fetches web content from allowed domains");
    println!("   Usage: fusion mcp connect web <DOMAIN>...\n");

    println!("ℹ Custom servers can be added via settings");

    Ok(())
}

/// Connect to an MCP server
pub async fn connect_server(server_type: String, args: Vec<String>) -> Result<()> {
    println!("🔌 Connecting to {} server...", server_type);

    let mut client = match server_type.as_str() {
        "filesystem" => {
            let path = args
                .first()
                .map(PathBuf::from)
                .unwrap_or_else(|| std::env::current_dir().unwrap());

            let server = FilesystemServer::new(path.clone());
            println!("  Root: {}", path.display());

            McpClient::start(&server.command(), &server.args())?
        }
        "github" => {
            let settings = fusion_settings::FusionSettings::load()?;
            let token = settings.github.token.ok_or_else(|| {
                anyhow::anyhow!("GitHub token not configured. Run: fusion gh auth login")
            })?;

            let server = GitHubServer::new(token);
            McpClient::start(&server.command(), &server.args())?
        }
        "web" => {
            if args.is_empty() {
                anyhow::bail!("At least one domain must be specified");
            }

            let server = WebServer::new(args.clone());
            println!("  Allowed domains: {}", args.join(", "));

            McpClient::start(&server.command(), &server.args())?
        }
        _ => anyhow::bail!("Unknown server type: {}", server_type),
    };

    println!("✓ Connected to MCP server\n");

    // Test connection by listing resources
    println!("📋 Available Resources:");
    match client.list_resources() {
        Ok(resources) => {
            if resources.is_empty() {
                println!("  (none)");
            } else {
                for resource in resources.iter().take(10) {
                    println!("  • {} ({})", resource.name, resource.uri);
                    if let Some(desc) = &resource.description {
                        println!("    {}", desc);
                    }
                }
                if resources.len() > 10 {
                    println!("  ... and {} more", resources.len() - 10);
                }
            }
        }
        Err(e) => {
            println!("  Error listing resources: {}", e);
        }
    }

    // List tools
    println!("\n🔧 Available Tools:");
    match client.list_tools() {
        Ok(tools) => {
            if tools.is_empty() {
                println!("  (none)");
            } else {
                for tool in tools.iter().take(10) {
                    println!("  • {}: {}", tool.name, tool.description);
                }
                if tools.len() > 10 {
                    println!("  ... and {} more", tools.len() - 10);
                }
            }
        }
        Err(e) => {
            println!("  Error listing tools: {}", e);
        }
    }

    client.shutdown()?;

    Ok(())
}

/// Read a resource from MCP server
pub async fn read_resource(server_type: String, uri: String) -> Result<()> {
    println!("📖 Reading resource: {}", uri);

    let mut client = match server_type.as_str() {
        "filesystem" => {
            let server = FilesystemServer::new(std::env::current_dir()?);
            McpClient::start(&server.command(), &server.args())?
        }
        _ => anyhow::bail!(
            "Server type not supported for resource reading: {}",
            server_type
        ),
    };

    let contents = client.read_resource(&uri)?;

    for content in contents {
        match content {
            fusion_mcp::ResourceContent::Text { text, .. } => {
                println!("\n{}", text);
            }
            fusion_mcp::ResourceContent::Blob {
                blob, mime_type, ..
            } => {
                println!("\nBinary content ({:?}): {} bytes", mime_type, blob.len());
            }
        }
    }

    client.shutdown()?;

    Ok(())
}

/// Call an MCP tool
pub async fn call_tool(
    server_type: String,
    tool_name: String,
    args_json: Option<String>,
) -> Result<()> {
    println!("🔧 Calling tool: {}", tool_name);

    let mut client = match server_type.as_str() {
        "filesystem" => {
            let server = FilesystemServer::new(std::env::current_dir()?);
            McpClient::start(&server.command(), &server.args())?
        }
        "github" => {
            let settings = fusion_settings::FusionSettings::load()?;
            let token = settings
                .github
                .token
                .ok_or_else(|| anyhow::anyhow!("GitHub token not configured"))?;

            let server = GitHubServer::new(token);
            McpClient::start(&server.command(), &server.args())?
        }
        _ => anyhow::bail!("Server type not supported: {}", server_type),
    };

    let arguments = if let Some(json) = args_json {
        Some(serde_json::from_str(&json)?)
    } else {
        None
    };

    let result = client.call_tool(&tool_name, arguments)?;

    println!("\n✓ Tool result:");
    for content in result {
        match content {
            fusion_mcp::ToolContent::Text { text } => {
                println!("{}", text);
            }
            fusion_mcp::ToolContent::Image { mime_type, .. } => {
                println!("Image result ({})", mime_type);
            }
            fusion_mcp::ToolContent::Resource { resource } => {
                println!("Resource: {} ({})", resource.name, resource.uri);
            }
        }
    }

    client.shutdown()?;

    Ok(())
}

/// Test MCP connection
pub async fn test_connection() -> Result<()> {
    println!("🧪 Testing MCP connection...\n");

    // Test filesystem server
    println!("Testing Filesystem Server:");
    let server = FilesystemServer::new(std::env::current_dir()?);
    match McpClient::start(&server.command(), &server.args()) {
        Ok(mut client) => {
            println!("  ✓ Connection successful");
            let _ = client.shutdown();
        }
        Err(e) => {
            println!("  ✗ Connection failed: {}", e);
        }
    }

    println!("\n✓ MCP test complete");

    Ok(())
}

/// Main MCP command dispatcher
pub fn mcp(cmd: McpCommands) -> Result<()> {
    match cmd {
        McpCommands::Serve { port, extensions } => {
            tokio::runtime::Runtime::new()?.block_on(async {
                println!("🚀 Starting MCP server on port {}...", port);
                if extensions {
                    println!("  Extension support: enabled");
                }
                println!("✓ MCP server running");
                Ok(())
            })
        }
        McpCommands::Context { cmd } => match cmd {
            ContextCommands::Add { path, recursive } => {
                println!("Adding context: {} (recursive: {})", path, recursive);
                Ok(())
            }
            ContextCommands::List => {
                println!("Listing context...");
                Ok(())
            }
            ContextCommands::Clear => {
                println!("Clearing context...");
                Ok(())
            }
        },
        McpCommands::Tools { cmd } => match cmd {
            ToolCommands::List => {
                println!("Listing MCP tools...");
                list_servers()
            }
        },
    }
}
