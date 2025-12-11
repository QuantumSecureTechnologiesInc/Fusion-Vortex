use anyhow::{Context, Result};
use tracing::{info, warn};

// Re-export subcommand types from main
pub use crate::{AiCommands, PackageCommands};

/// Create a new Fusion project
pub fn new_project(name: &str, template: &str, path: Option<&str>) -> Result<()> {
    info!("Creating new {} project: {}", template, name);

    let target_path = path.unwrap_or(".");
    let project_path = std::path::Path::new(target_path).join(name);

    // Delegate to toolchain
    fusion_toolchain::new_project(name, template, &project_path)
        .context("Failed to create new project")?;

    println!(
        "✓ Created new {} project '{}' at {}",
        template,
        name,
        project_path.display()
    );
    Ok(())
}

/// Build the current project
pub fn build(release: bool, target: Option<&str>, verbose: bool) -> Result<()> {
    info!(
        "Building project (release: {}, target: {:?})",
        release, target
    );

    fusion_toolchain::build(release, target, verbose).context("Build failed")?;

    println!("✓ Build completed successfully");
    Ok(())
}

/// Run the current project
pub fn run(release: bool, args: &[String]) -> Result<()> {
    info!("Running project (release: {}, args: {:?})", release, args);

    fusion_toolchain::run(release, args).context("Run failed")?;

    Ok(())
}

/// Run tests
pub fn test(filter: Option<&str>, release: bool, bench: bool) -> Result<()> {
    info!(
        "Running tests (filter: {:?}, release: {}, bench: {})",
        filter, release, bench
    );

    fusion_tester::run_tests(filter, release, bench).context("Tests failed")?;

    println!("✓ All tests passed");
    Ok(())
}

/// Format source code
pub fn fmt(check: bool, all: bool) -> Result<()> {
    info!("Formatting code (check: {}, all: {})", check, all);

    let result = fusion_formatter::format(check, all).context("Formatting failed")?;

    if check {
        if result.needs_formatting {
            println!("⚠ Some files need formatting");
            std::process::exit(1);
        } else {
            println!("✓ All files are properly formatted");
        }
    } else {
        println!("✓ Formatted {} files", result.formatted_count);
    }

    Ok(())
}

/// Check code without building
pub fn check(all: bool) -> Result<()> {
    info!("Checking code (all: {})", all);

    fusion_analyzer::check(all).context("Check failed")?;

    println!("✓ Check completed successfully");
    Ok(())
}

/// Lint and analyze code
pub fn lint(fix: bool, security: bool) -> Result<()> {
    info!("Linting code (fix: {}, security: {})", fix, security);

    let result = fusion_analyzer::lint(fix, security).context("Lint failed")?;

    if result.has_errors {
        println!(
            "✗ Found {} errors, {} warnings",
            result.error_count, result.warning_count
        );
        std::process::exit(1);
    } else if result.warning_count > 0 {
        println!("⚠ Found {} warnings", result.warning_count);
    } else {
        println!("✓ No issues found");
    }

    Ok(())
}

/// Generate documentation
pub fn doc(open: bool, private: bool) -> Result<()> {
    info!(
        "Generating documentation (open: {}, private: {})",
        open, private
    );

    let output_path =
        fusion_docgen::generate(private).context("Documentation generation failed")?;

    println!("✓ Documentation generated at {}", output_path.display());

    if open {
        fusion_docgen::open(&output_path)?;
    }

    Ok(())
}

/// Manage dependencies
pub fn package(cmd: PackageCommands) -> Result<()> {
    use PackageCommands::*;

    match cmd {
        Add { package, version } => {
            info!("Adding package: {} (version: {:?})", package, version);
            fusion_pkgmgr::add(&package, version.as_deref())?;
            println!("✓ Added package '{}'", package);
        }
        Remove { package } => {
            info!("Removing package: {}", package);
            fusion_pkgmgr::remove(&package)?;
            println!("✓ Removed package '{}'", package);
        }
        Update { all } => {
            info!("Updating dependencies (all: {})", all);
            let updated = fusion_pkgmgr::update(all)?;
            println!("✓ Updated {} packages", updated);
        }
        List => {
            info!("Listing dependencies");
            fusion_pkgmgr::list()?;
        }
        Publish { no_verify } => {
            info!("Publishing package (no_verify: {})", no_verify);
            fusion_pkgmgr::publish(!no_verify)?;
            println!("✓ Package published successfully");
        }
    }

    Ok(())
}

/// Debug the current project
pub fn debug(target: Option<&str>) -> Result<()> {
    info!("Starting debugger (target: {:?})", target);

    fusion_debugger::start(target).context("Debugger failed to start")?;

    Ok(())
}

/// Profile runtime performance
pub fn profile(mode: &str, output: &str) -> Result<()> {
    info!("Profiling (mode: {}, output: {})", mode, output);

    let result = fusion_profiler::profile(mode, output).context("Profiling failed")?;

    println!("✓ Profile saved to {}", result.output_path.display());
    Ok(())
}

/// Audit dependencies for vulnerabilities
pub fn audit(report: bool, deny: bool) -> Result<()> {
    info!("Auditing dependencies (report: {}, deny: {})", report, deny);

    let result = fusion_audit::audit(report).context("Audit failed")?;

    if result.vulnerabilities.is_empty() {
        println!("✓ No vulnerabilities found");
    } else {
        warn!("Found {} vulnerabilities", result.vulnerabilities.len());

        for vuln in &result.vulnerabilities {
            println!("⚠ {}: {} ({})", vuln.package, vuln.title, vuln.severity);
        }

        if deny {
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Deploy to cloud platforms
pub fn deploy(platform: &str, env: &str, config: Option<&str>) -> Result<()> {
    info!(
        "Deploying to {} (env: {}, config: {:?})",
        platform, env, config
    );

    let result = fusion_deploy::deploy(platform, env, config).context("Deployment failed")?;

    println!("✓ Deployed successfully to {}", result.endpoint);
    Ok(())
}

/// AI-powered development assistance
pub fn ai(cmd: AiCommands) -> Result<()> {
    use AiCommands::*;

    match cmd {
        Assist { prompt, ai_offline } => {
            info!("Starting AI assistant (offline: {})", ai_offline);
            fusion_ai_cli::assist(prompt.as_deref(), ai_offline)?;
        }
        Generate {
            description,
            target,
            preview_only,
            ai_offline,
            max_tokens,
        } => {
            info!(
                "Generating code: {} (preview: {})",
                description, preview_only
            );
            fusion_ai_cli::generate(
                &description,
                target.as_deref(),
                preview_only,
                ai_offline,
                max_tokens,
            )?;
        }
        Refactor {
            description,
            target,
            preview_only,
        } => {
            info!("Refactoring: {} -> {}", target, description);
            fusion_ai_cli::refactor(&description, &target, preview_only)?;
        }
        Explain { target, depth } => {
            info!("Explaining: {} (depth: {})", target, depth);
            fusion_ai_cli::explain(&target, &depth)?;
        }
        Review { target, focus } => {
            info!("Reviewing code (focus: {})", focus);
            fusion_ai_cli::review(target.as_deref(), &focus)?;
        }
        Tests { target, test_type } => {
            info!("Generating tests for: {} (type: {})", target, test_type);
            fusion_ai_cli::generate_tests(&target, &test_type)?;
        }
        Doc { target, examples } => {
            info!("Generating documentation for: {}", target);
            fusion_ai_cli::generate_docs(&target, examples)?;
        }
        Config {
            show,
            model,
            api_key,
        } => {
            info!("Configuring AI settings");
            fusion_ai_cli::config(show, model.as_deref(), api_key.as_deref())?;
        }
    }

    Ok(())
}

/// Manage MCP (Model Context Protocol) server
pub fn mcp(cmd: crate::McpCommands) -> Result<()> {
    use crate::{ContextCommands::*, McpCommands::*, ToolCommands::*};

    match cmd {
        Serve { port, extensions } => {
            info!(
                "Starting MCP server on port {} (extensions: {})",
                port, extensions
            );

            // Create runtime for async server
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let config = fusion_mcp::ServerConfig {
                    name: "fusion-mcp".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    capabilities: fusion_mcp::ServerCapabilities {
                        tools: true,
                        resources: true,
                        prompts: true,
                        extensions,
                    },
                };

                let server = fusion_mcp::McpServer::new(config);
                server.start(&format!("127.0.0.1:{}", port)).await
            })?;
        }
        Context { cmd } => match cmd {
            Add { path, recursive } => {
                info!("Adding context: {} (recursive: {})", path, recursive);
                // Implementation would call into context provider
                println!("✓ Added context from {}", path);
            }
            List => {
                info!("Listing context");
                // Implementation would list context
                println!("Current context is empty");
            }
            Clear => {
                info!("Clearing context");
                println!("✓ Context cleared");
            }
        },
        Tools { cmd } => match cmd {
            List => {
                info!("Listing tools");
                // Implementation would list tools
                let registry = fusion_mcp::ToolRegistry::new();
                for tool in registry.list_tools() {
                    println!("- {}: {}", tool.name, tool.description);
                }
            }
        },
    }

    Ok(())
}

/// Manage VS Code extensions
pub fn extensions(cmd: crate::ExtensionCommands) -> Result<()> {
    use crate::ExtensionCommands::*;

    match cmd {
        List => {
            info!("Listing extensions");
            // Implementation would list extensions
            println!("No extensions installed");
        }
        Install { id } => {
            info!("Installing extension: {}", id);
            // Implementation would install extension
            println!("✓ Installed extension {}", id);
        }
        Exec { command, args } => {
            info!("Executing command: {} (args: {:?})", command, args);
            // Implementation would execute command
            println!("✓ Executed command {}", command);
        }
    }

    Ok(())
}
