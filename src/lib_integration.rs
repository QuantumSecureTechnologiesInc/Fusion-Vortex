// src/lib.rs - Updated with package manager integration

pub mod ast;
pub mod borrow_checker;
pub mod codegen;
pub mod lexer;
pub mod lsp;
pub mod module_resolver;
pub mod package_manager; // NEW: Package manager module
pub mod parser;
pub mod semantic_analyzer;
pub mod stdlib;
pub mod wasm;

use std::path::Path;

/// Compile a Fusion source file with package management support
pub fn compile_with_packages(
    source_path: &Path,
    output_path: &Path,
    target: CompileTarget,
    use_packages: bool,
) -> Result<(), String> {
    // Step 1: If using packages, resolve dependencies
    if use_packages {
        use package_manager::PackageManager;
        use std::env;

        let cache_dir = env::var("FUSION_CACHE").unwrap_or_else(|_| {
            env::home_dir()
                .unwrap()
                .join(".fusion/cache")
                .to_string_lossy()
                .to_string()
        });

        let mut pm = PackageManager::new(cache_dir.into());

        // Read fusion.toml if exists
        let manifest_path = source_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("fusion.toml");

        if manifest_path.exists() {
            println!("📦 Resolving dependencies from {}", manifest_path.display());
            // TODO: Parse manifest and install dependencies
            // pm.install_from_manifest(&manifest_path)?;
        }
    }

    // Step 2: Continue with normal compilation
    compile_file(source_path, output_path, target)
}

#[derive(Debug, Clone, Copy)]
pub enum CompileTarget {
    LLVM,
    WebAssembly,
}

/// Existing compile function
pub fn compile_file(
    source_path: &Path,
    output_path: &Path,
    target: CompileTarget,
) -> Result<(), String> {
    // Existing compilation logic
    Ok(())
}
