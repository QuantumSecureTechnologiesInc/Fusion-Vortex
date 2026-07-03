// preprocess.rs — Fusion self-hosting preprocessor
// Ported from scripts/selfhost_preprocess.ps1 for cross-platform support.
//
// Handles: recursive module resolution, pub stripping, mod/use removal,
//          const inlining, extern dedup, hex literal conversion (string-aware)

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Dependency order for topological sort when flattening modules.
/// Modules earlier in this list appear first in the flattened output.
const DEPENDENCY_ORDER: &[&str] = &[
    "ast",
    "lexer",
    "parser",
    "sema",
    "ir",
    "codegen",
    "wasm_encoder",
    "wasm_types",
    "wasm",
    "optimizer",
    "cli",
    "stage1_parser_api",
    "stage1_sema_api",
    "dummy",
    "chaos_vacuum",
];

/// Modules to skip (aspirational or orchestrators)
const SKIP_MODULES: &[&str] = &["pure_fusion_compiler", "llvm", "dwarf", "lib"];

/// Preprocess a Fusion source file.
///
/// Pipeline: source.fu → resolve mods → strip pub → remove mod/use/const →
///           inline consts → dedup externs → convert hex → write output
pub fn preprocess(input_path: &Path, output_path: &Path, resolve_modules: bool) -> Result<(), String> {
    let lines = if resolve_modules {
        resolve_and_flatten(input_path)?
    } else {
        read_lines(input_path)?
    };

    let consts = collect_consts(&lines);
    let mut seen_externs: HashSet<String> = HashSet::new();
    let mut output: Vec<String> = Vec::new();

    let mod_re = Regex::new(r"^(mod\s+\w+\s*;|use\s+\S+\s*;|const\s+\w+\s*:\s*\w+\s*=)").unwrap();
    let extern_re = Regex::new(r"^extern\s+fn\s+(\w+)\s*\(").unwrap();

    for line in &lines {
        let trimmed = line.trim_start();
        let mut stripped = line.clone();

        // Stage 1: Strip 'pub ' keyword
        let pub_re = Regex::new(r"\bpub\s+").unwrap();
        stripped = pub_re.replace_all(&stripped, "").to_string();

        // Stage 2: Skip mod/use/const declarations entirely
        if mod_re.is_match(trimmed) {
            continue;
        }

        // Stage 3: Inline const references
        for (name, value) in &consts {
            let const_re = Regex::new(&format!(r"\b{}\b", regex::escape(name))).unwrap();
            stripped = const_re.replace_all(&stripped, value.as_str()).to_string();
        }

        // Stage 4: Deduplicate extern fn declarations
        if let Some(caps) = extern_re.captures(trimmed) {
            let fn_name = caps[1].to_string();
            if seen_externs.contains(&fn_name) {
                continue;
            }
            seen_externs.insert(fn_name);
        }

        // Stage 5: Convert hex literals to decimal (string-aware)
        stripped = convert_hex_literals(&stripped);

        output.push(stripped);
    }

    // Write flattened output
    let content = output.join("\n") + "\n";
    fs::write(output_path, content.as_bytes())
        .map_err(|e| format!("Failed to write output: {}", e))?;

    eprintln!(
        "  [preprocess] {} -> {} ({} lines)",
        input_path.display(),
        output_path.display(),
        output.len()
    );

    Ok(())
}

/// Recursively resolve mod declarations and flatten into a single source.
fn resolve_and_flatten(input_path: &Path) -> Result<Vec<String>, String> {
    let root_dir = input_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    let mut resolved_paths: HashMap<String, PathBuf> = HashMap::new();

    // Scan root file for mod declarations
    let root_lines = read_lines(input_path)?;
    for line in &root_lines {
        let trimmed = line.trim_start();
        if let Some(mod_name) = parse_mod_declaration(trimmed) {
            resolve_module_files(&root_dir, &mod_name, &mut resolved_paths)?;
        }
    }

    // Collect non-module content from root file
    let mut root_non_mod_lines: Vec<String> = Vec::new();
    let mod_use_re = Regex::new(r"^(mod\s+\w+\s*;|use\s+\S+\s*;)").unwrap();
    for line in &root_lines {
        let trimmed = line.trim_start();
        if !mod_use_re.is_match(trimmed) {
            root_non_mod_lines.push(line.clone());
        }
    }

    // Concatenate resolved modules in dependency order
    let mut all_lines: Vec<String> = Vec::new();
    for mod_name in DEPENDENCY_ORDER {
        if let Some(mod_path) = resolved_paths.get(*mod_name) {
            let mod_content = read_lines(mod_path)?;
            all_lines.push(format!(
                "// === begin module: {} ({}) ===",
                mod_name,
                mod_path.display()
            ));
            all_lines.extend(mod_content);
            all_lines.push(format!("// === end module: {} ===", mod_name));
        }
    }

    // Append root file's non-module content
    if !root_non_mod_lines.is_empty() {
        all_lines.push(format!("// === begin root: {} ===", input_path.display()));
        all_lines.extend(root_non_mod_lines);
        all_lines.push("// === end root ===".to_string());
    }

    eprintln!(
        "  [mod] resolved {} modules, {} total lines",
        resolved_paths.len(),
        all_lines.len()
    );

    Ok(all_lines)
}

/// Recursively discover module files starting from root directory.
fn resolve_module_files(
    root_dir: &Path,
    module_name: &str,
    resolved: &mut HashMap<String, PathBuf>,
) -> Result<(), String> {
    if resolved.contains_key(module_name) {
        return Ok(());
    }
    if SKIP_MODULES.contains(&module_name) {
        return Ok(());
    }

    let fu_path = root_dir.join(format!("{}.fu", module_name));
    let mod_dir = root_dir.join(module_name);
    let mod_fu_path = mod_dir.join("mod.fu");

    let found_path = if fu_path.exists() {
        fu_path
    } else if mod_fu_path.exists() {
        mod_fu_path
    } else {
        return Ok(()); // Module not found, skip silently
    };

    resolved.insert(module_name.to_string(), found_path.clone());

    // Recursively scan this file for its own mod declarations
    let sub_lines = read_lines(&found_path)?;
    let sub_dir = found_path.parent().unwrap_or(root_dir);
    for line in &sub_lines {
        let trimmed = line.trim_start();
        if let Some(sub_name) = parse_mod_declaration(trimmed) {
            resolve_module_files(sub_dir, &sub_name, resolved)?;
        }
    }

    Ok(())
}

/// Parse a `mod X;` declaration, returning the module name.
fn parse_mod_declaration(line: &str) -> Option<String> {
    let re = Regex::new(r"^mod\s+(\w+)\s*;").unwrap();
    re.captures(line.trim_start())
        .map(|caps| caps[1].to_string())
}

/// Collect const definitions from lines.
fn collect_consts(lines: &[String]) -> HashMap<String, String> {
    let mut consts = HashMap::new();
    let re = Regex::new(r"^const\s+(\w+)\s*:\s*\w+\s*=\s*(.+?)\s*;").unwrap();
    for line in lines {
        let trimmed = line.trim_start();
        if let Some(caps) = re.captures(trimmed) {
            let name = caps[1].to_string();
            let value = caps[2].trim_end_matches(';').trim().to_string();
            consts.insert(name, value);
        }
    }
    consts
}

/// Convert hex literals (0xNN) to decimal, but NOT inside string literals.
fn convert_hex_literals(line: &str) -> String {
    let hex_re = Regex::new(r"\b0x([0-9a-fA-F]+)\b").unwrap();

    // Split on quotes: even-indexed segments are outside strings, odd-indexed are inside.
    let parts: Vec<&str> = line.split('"').collect();
    let mut result = String::new();

    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            result.push('"');
        }
        if i % 2 == 0 {
            // Outside string: safe to convert hex literals
            let converted = hex_re.replace_all(part, |caps: &regex::Captures| {
                u64::from_str_radix(&caps[1], 16)
                    .map(|n| n.to_string())
                    .unwrap_or_else(|_| caps[0].to_string())
            });
            result.push_str(&converted);
        } else {
            // Inside string: leave as-is
            result.push_str(part);
        }
    }

    result
}

/// Read all lines from a file.
fn read_lines(path: &Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mod_declaration() {
        assert_eq!(parse_mod_declaration("mod ast;"), Some("ast".to_string()));
        assert_eq!(parse_mod_declaration("mod parser;"), Some("parser".to_string()));
        assert_eq!(parse_mod_declaration("  mod lexer;"), Some("lexer".to_string()));
        assert_eq!(parse_mod_declaration("fn main() -> int {"), None);
        assert_eq!(parse_mod_declaration("use std::io;"), None);
    }

    #[test]
    fn test_convert_hex_literals() {
        assert_eq!(convert_hex_literals("0x10"), "16");
        assert_eq!(convert_hex_literals("0xFF"), "255");
        assert_eq!(convert_hex_literals("0x0"), "0");
        // Inside strings should be preserved
        assert_eq!(convert_hex_literals(r#""hello 0xFF world""#), r#""hello 0xFF world""#);
        // Mixed
        assert_eq!(
            convert_hex_literals(r#"let x = 0x10; printf("0x10");"#),
            r#"let x = 16; printf("0x10");"#
        );
    }

    #[test]
    fn test_collect_consts() {
        let lines = vec![
            "const FOO: int = 42;".to_string(),
            "const BAR: string = \"hello\";".to_string(),
            "fn main() -> int { return FOO; }".to_string(),
        ];
        let consts = collect_consts(&lines);
        assert_eq!(consts.get("FOO").unwrap(), "42");
        assert_eq!(consts.get("BAR").unwrap(), "\"hello\"");
    }
}