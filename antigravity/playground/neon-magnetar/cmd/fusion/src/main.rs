use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod commands;

/// Fusion Programming Language CLI
///
/// A next-generation programming language with built-in AI assistance,
/// post-quantum cryptography, and enterprise-grade tooling.
#[derive(Parser, Debug)]
#[command(
    name = "fusion",
    version,
    about = "Fusion Programming Language CLI",
    long_about = "Fusion is a next-generation programming language with AI assistance, \
                  quantum-resistant cryptography, and production-ready tooling.",
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
        
        /// Project template (binary, library, quantum, ai-app)
        #[arg(short, long, default_value = "binary")]
        template: String,
        
        /// Target directory
        #[arg(short = 'p', long)]
        path: Option<String>,
    },

    /// Build the current project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
        
        /// Target architecture
        #[arg(short, long)]
        target: Option<String>,
        
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run the current project
    Run {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
        
        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Run tests
    Test {
        /// Test filter pattern
        filter: Option<String>,
        
        /// Run tests in release mode
        #[arg(short, long)]
        release: bool,
        
        /// Enable benchmarking
        #[arg(short, long)]
        bench: bool,
    },

    /// Format source code
    Fmt {
        /// Check formatting without modifying files
        #[arg(short, long)]
        check: bool,
        
        /// Format all files in workspace
        #[arg(short, long)]
        all: bool,
    },

    /// Check code without building
    Check {
        /// Check all targets
        #[arg(short, long)]
        all: bool,
    },

    /// Lint and analyze code
    Lint {
        /// Fix automatically fixable issues
        #[arg(short, long)]
        fix: bool,
        
        /// Enable security-focused lints
        #[arg(short, long)]
        security: bool,
    },

    /// Generate documentation
    Doc {
        /// Open documentation in browser
        #[arg(short, long)]
        open: bool,
        
        /// Include private items
        #[arg(short, long)]
        private: bool,
    },

    /// Manage dependencies
    Package {
        #[command(subcommand)]
        cmd: PackageCommands,
    },

    /// Debug the current project
    Debug {
        /// Entry point to debug
        target: Option<String>,
    },

    /// Profile runtime performance
    Profile {
        /// Profiling mode (cpu, memory, gpu)
        #[arg(short, long, default_value = "cpu")]
        mode: String,
        
        /// Output format (json, flamegraph, trace)
        #[arg(short, long, default_value = "flamegraph")]
        output: String,
    },

    /// Audit dependencies for vulnerabilities
    Audit {
        /// Generate detailed report
        #[arg(short, long)]
        report: bool,
        
        /// Fail on vulnerabilities
        #[arg(short, long)]
        deny: bool,
    },

    /// Deploy to cloud platforms
    Deploy {
        /// Target platform (aws, azure, gcp, local)
        #[arg(short, long)]
        platform: String,
        
        /// Deployment environment (dev, staging, production)
        #[arg(short, long, default_value = "dev")]
        env: String,
        
        /// Configuration file
        #[arg(short, long)]
        config: Option<String>,
    },

    /// AI-powered development assistance
    Ai {
        #[command(subcommand)]
        cmd: AiCommands,
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
    },
    
    /// Explain code
    Explain {
        /// Code file or selection
        target: String,
        
        /// Explanation depth (quick, detailed, comprehensive)
        #[arg(short, long, default_value = "detailed")]
        depth: String,
    },
    
    /// Review code for issues
    Review {
        /// Target to review
        target: Option<String>,
        
        /// Focus areas (security, performance, style, all)
        #[arg(short, long, default_value = "all")]
        focus: String,
    },
    
    /// Generate tests
    Tests {
        /// Target code to test
        target: String,
        
        /// Test type (unit, integration, e2e)
        #[arg(short = 't', long, default_value = "unit")]
        test_type: String,
    },
    
    /// Generate documentation
    Doc {
        /// Target to document
        target: String,
        
        /// Include examples
        #[arg(short, long)]
        examples: bool,
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
        Commands::New { name, template, path } => {
            commands::new_project(&name, &template, path.as_deref())
        }
        Commands::Build { release, target, verbose } => {
            commands::build(release, target.as_deref(), verbose)
        }
        Commands::Run { release, args } => {
            commands::run(release, &args)
        }
        Commands::Test { filter, release, bench } => {
            commands::test(filter.as_deref(), release, bench)
        }
        Commands::Fmt { check, all } => {
            commands::fmt(check, all)
        }
        Commands::Check { all } => {
            commands::check(all)
        }
        Commands::Lint { fix, security } => {
            commands::lint(fix, security)
        }
        Commands::Doc { open, private } => {
            commands::doc(open, private)
        }
        Commands::Package { cmd } => {
            commands::package(cmd)
        }
        Commands::Debug { target } => {
            commands::debug(target.as_deref())
        }
        Commands::Profile { mode, output } => {
            commands::profile(&mode, &output)
        }
        Commands::Audit { report, deny } => {
            commands::audit(report, deny)
        }
        Commands::Deploy { platform, env, config } => {
            commands::deploy(&platform, &env, config.as_deref())
        }
        Commands::Ai { cmd } => {
            commands::ai(cmd)
        }
    }
}
