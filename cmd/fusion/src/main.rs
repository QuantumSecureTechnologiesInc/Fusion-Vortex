use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod commands;

/// Fusion Programming Language CLI
///
/// A next-generation programming language with built-in AI assistance,
/// post-quantum cryptography, and enterprise-grade tooling.
#[derive(Parser)]
#[command(
    name = "fusion-vsc",
    version,
    about = "Fusion VSC CLI",
    long_about = "Fusion VSC CLI - The bridges between Fusion, VS Code, and MCP.",
    author,
    propagate_version = true
)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Enable debug logging
    #[arg(short, long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new Fusion project
    New {
        /// Project name
        name: String,

        /// Template to use
        #[arg(short, long, default_value = "default")]
        template: String,

        /// Custom project path
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Build for target
        #[arg(short, long)]
        target: Option<String>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run the project
    Run {
        /// Run in release mode
        #[arg(short, long)]
        release: bool,

        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Run tests
    Test {
        /// Test name filter
        #[arg(short, long)]
        filter: Option<String>,

        /// Run in release mode
        #[arg(short, long)]
        release: bool,

        /// Run benchmarks
        #[arg(short, long)]
        bench: bool,
    },

    /// Format source code
    Fmt {
        /// Check formatting without modifying files
        #[arg(short, long)]
        check: bool,

        /// Format all files
        #[arg(short, long)]
        all: bool,
    },

    /// Check source code
    Check {
        /// Check all packages
        #[arg(short, long)]
        all: bool,
    },

    /// Lint source code
    Lint {
        /// Auto-fix issues
        #[arg(short, long)]
        fix: bool,

        /// Security-focused linting
        #[arg(short, long)]
        security: bool,
    },

    /// Generate documentation
    Doc {
        /// Open docs in browser
        #[arg(short, long)]
        open: bool,

        /// Include private items
        #[arg(short, long)]
        private: bool,
    },

    /// Package management
    Package {
        #[command(subcommand)]
        cmd: PackageCommands,
    },

    /// Debug the project
    Debug {
        /// Debug target
        target: Option<String>,
    },

    /// Profile the project
    Profile {
        /// Profiling mode (cpu, memory, time)
        #[arg(short, long, default_value = "cpu")]
        mode: String,

        /// Output file
        #[arg(short, long)]
        output: String,
    },

    /// Security audit
    Audit {
        /// Generate audit report
        #[arg(short, long)]
        report: bool,

        /// Deny warnings
        #[arg(short, long)]
        deny: bool,
    },

    /// Deploy the project
    Deploy {
        /// Target platform
        #[arg(short, long)]
        platform: String,

        /// Deployment environment
        #[arg(short, long, default_value = "production")]
        env: String,

        /// Config file
        #[arg(short, long)]
        config: Option<String>,
    },

    /// AI-powered development tools
    Ai {
        #[command(subcommand)]
        cmd: AiCommands,
    },

    /// Model Context Protocol commands
    Mcp {
        #[command(subcommand)]
        cmd: McpCommands,
    },

    /// VS Code extension management
    Extensions {
        #[command(subcommand)]
        cmd: ExtensionCommands,
    },
}

#[derive(Subcommand, Debug)]
enum FusionCommands {
    /// Execute the Flux‑Resolve Engine – a GPU‑accelerated dependency resolver.
    #[command(name = "flux-resolve")]
    FluxResolve {
        /// Path to the project manifest (e.g. `fusion.toml`).
        #[arg(short, long)]
        manifest: Option<String>,
    },
    /// Upgrade the Fusion Runtime Core to the latest version.
    #[command(name = "runtime-upgrade")]
    RuntimeUpgrade {
        /// Target version (defaults to the latest stable release).
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Placeholder for legacy commands – retained for backward compatibility.
    #[command(name = "legacy")]
    Legacy {
        /// The original command name to invoke.
        #[arg()]
        cmd: String,
        /// Additional arguments passed through.
        #[arg(last = true)]
        args: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
enum McpCommands {
    /// Start MCP server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Enable extension support
        #[arg(short, long)]
        extensions: bool,
    },

    /// Manage context
    Context {
        #[command(subcommand)]
        cmd: ContextCommands,
    },

    /// Manage tools
    Tools {
        #[command(subcommand)]
        cmd: ToolCommands,
    },
}

#[derive(Subcommand, Debug)]
enum ContextCommands {
    /// Add context
    Add {
        /// Path to file or directory
        path: String,

        /// Recursive for directories
        #[arg(short, long)]
        recursive: bool,
    },

    /// List context
    List,

    /// Clear context
    Clear,
}

#[derive(Subcommand, Debug)]
enum ToolCommands {
    /// List available tools
    List,
}

#[derive(Subcommand, Debug)]
enum ExtensionCommands {
    /// List installed extensions
    List,

    /// Install an extension
    Install {
        /// Extension ID (publisher.name)
        id: String,
    },

    /// Execute extension command
    Exec {
        /// Command ID
        command: String,

        /// Arguments (JSON)
        #[arg(short, long)]
        args: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum PackageCommands {
    /// Add a dependency
    Add {
        /// Package name
        package: String,

        /// Package version
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },

    /// Update dependencies
    Update {
        /// Update all dependencies
        #[arg(short, long)]
        all: bool,
    },

    /// List dependencies
    List,

    /// Publish package
    Publish {
        /// Skip verification
        #[arg(long)]
        no_verify: bool,
    },
}

#[derive(Subcommand, Debug)]
enum AiCommands {
    /// Interactive AI assistant
    Assist {
        /// Initial prompt
        prompt: Option<String>,

        /// Use offline/local models only
        #[arg(long)]
        ai_offline: bool,
    },

    /// Generate code from description
    Generate {
        /// Description of code to generate
        description: String,

        /// Target file or directory
        #[arg(short, long)]
        target: Option<String>,

        /// Preview only (don't apply)
        #[arg(short, long)]
        preview_only: bool,

        /// Offline mode
        #[arg(long)]
        ai_offline: bool,

        /// Maximum tokens
        #[arg(long)]
        max_tokens: Option<usize>,
    },

    /// Refactor existing code
    Refactor {
        /// Refactoring description
        description: String,

        /// Target code selection
        #[arg(short, long)]
        target: String,

        /// Preview only
        #[arg(short, long)]
        preview_only: bool,

        /// Use offline/local models only
        #[arg(long)]
        ai_offline: bool,
    },

    /// Explain code
    Explain {
        /// Code file or selection
        target: String,

        /// Explanation depth (quick, detailed, comprehensive)
        #[arg(short, long, default_value = "detailed")]
        depth: String,

        /// Use offline/local models only
        #[arg(long)]
        ai_offline: bool,
    },

    /// Review code for issues
    Review {
        /// Target to review
        target: Option<String>,

        /// Focus areas (security, performance, style, all)
        #[arg(short, long, default_value = "all")]
        focus: String,

        /// Use offline/local models only
        #[arg(long)]
        ai_offline: bool,
    },

    /// Generate tests
    Tests {
        /// Target code to test
        target: String,

        /// Test type (unit, integration, e2e)
        #[arg(short = 't', long, default_value = "unit")]
        test_type: String,

        /// Use offline/local models only
        #[arg(long)]
        ai_offline: bool,
    },

    /// Generate documentation
    Doc {
        /// Target to document
        target: String,

        /// Include examples
        #[arg(short, long)]
        examples: bool,

        /// Use offline/local models only
        #[arg(long)]
        ai_offline: bool,
    },

    /// Configure AI settings
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,

        /// Set default model
        #[arg(long)]
        model: Option<String>,

        /// Set API key
        #[arg(long)]
        api_key: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.debug {
        Level::DEBUG
    } else if cli.verbose {
        Level::INFO
    } else {
        Level::WARN
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(cli.debug)
        .with_line_number(cli.debug)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Fusion CLI v{}", env!("CARGO_PKG_VERSION"));

    // Dispatch commands
    match cli.command {
        Commands::New {
            name,
            template,
            path,
        } => commands::project::new_project(&name, &template, path.as_deref()),
        Commands::Build {
            release,
            target,
            verbose,
        } => commands::build::build(release, target.as_deref(), verbose),
        Commands::Run { release, args } => commands::run::run(release, &args),
        Commands::Test {
            filter,
            release,
            bench,
        } => commands::test::test(filter.as_deref(), release, bench),
        Commands::Fmt { check, all } => commands::fmt::fmt(check, all),
        Commands::Check { all } => commands::check::check(all),
        Commands::Lint { fix, security } => commands::lint::lint(fix, security),
        Commands::Doc { open, private } => commands::doc::doc(open, private),
        Commands::Package { cmd } => commands::package::package(cmd),
        Commands::Debug { target } => commands::debug::debug(target.as_deref()),
        Commands::Profile { mode, output } => commands::profile::profile(&mode, &output),
        Commands::Audit { report, deny } => commands::audit::audit(report, deny),
        Commands::Deploy {
            platform,
            env,
            config,
        } => commands::deploy::deploy(&platform, &env, config.as_deref()),
        Commands::Ai { cmd } => commands::ai::ai(cmd),
        Commands::Mcp { cmd } => commands::mcp::mcp(cmd),
        Commands::Extensions { cmd } => commands::extensions::extensions(cmd),
    }
}
