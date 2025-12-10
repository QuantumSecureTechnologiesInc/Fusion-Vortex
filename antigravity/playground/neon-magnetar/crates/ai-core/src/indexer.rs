use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Codebase indexer for fast code search
pub struct CodebaseIndex {
    files: HashMap<PathBuf, FileIndex>,
    symbols: HashMap<String, Vec<SymbolLocation>>,
}

#[derive(Debug, Clone)]
struct FileIndex {
    path: PathBuf,
    content: String,
    symbols: Vec<Symbol>,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    kind: SymbolKind,
    line: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Module,
    Const,
    Type,
}

#[derive(Debug, Clone)]
pub struct SymbolLocation {
    file: PathBuf,
    line: usize,
    col: usize,
    context: String,
}

impl CodebaseIndex {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    /// Index a directory recursively
    pub fn index_directory(&mut self, root: &Path) -> Result<()> {
        tracing::info!("Indexing directory: {}", root.display());

        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Only index source files
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("rs" | "fu" | "toml" | "md")) {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        self.index_file(path.to_path_buf(), content)?;
                    }
                }
            }
        }

        tracing::info!(
            "Indexed {} files, {} symbols",
            self.files.len(),
            self.symbols.len()
        );

        Ok(())
    }

    /// Index a single file
    fn index_file(&mut self, path: PathBuf, content: String) -> Result<()> {
        let symbols = self.extract_symbols(&content);
        let dependencies = self.extract_dependencies(&content);

        // Store file index
        self.files.insert(
            path.clone(),
            FileIndex {
                path: path.clone(),
                content: content.clone(),
                symbols: symbols.clone(),
                dependencies,
            },
        );

        // Index symbols
        for symbol in symbols {
            let location = SymbolLocation {
                file: path.clone(),
                line: symbol.line,
                col: symbol.col,
                context: self.get_context(&content, symbol.line),
            };

            self.symbols
                .entry(symbol.name.clone())
                .or_insert_with(Vec::new)
                .push(location);
        }

        Ok(())
    }

    /// Extract symbols from code
    fn extract_symbols(&self, content: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Rust function
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                if let Some(name) = self.extract_name(trimmed, "fn ") {
                    symbols.push(Symbol {
                        name,
                        kind: SymbolKind::Function,
                        line: line_num + 1,
                        col: line.find("fn ").unwrap_or(0),
                    });
                }
            }

            // Rust struct
            if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") {
                if let Some(name) = self.extract_name(trimmed, "struct ") {
                    symbols.push(Symbol {
                        name,
                        kind: SymbolKind::Struct,
                        line: line_num + 1,
                        col: line.find("struct ").unwrap_or(0),
                    });
                }
            }

            // Rust enum
            if trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") {
                if let Some(name) = self.extract_name(trimmed, "enum ") {
                    symbols.push(Symbol {
                        name,
                        kind: SymbolKind::Enum,
                        line: line_num + 1,
                        col: line.find("enum ").unwrap_or(0),
                    });
                }
            }

            // Rust trait
            if trimmed.starts_with("trait ") || trimmed.starts_with("pub trait ") {
                if let Some(name) = self.extract_name(trimmed, "trait ") {
                    symbols.push(Symbol {
                        name,
                        kind: SymbolKind::Trait,
                        line: line_num + 1,
                        col: line.find("trait ").unwrap_or(0),
                    });
                }
            }
        }

        symbols
    }

    fn extract_name(&self, line: &str, keyword: &str) -> Option<String> {
        line.find(keyword)
            .map(|idx| &line[idx + keyword.len()..])
            .and_then(|rest| rest.split_whitespace().next())
            .map(|s| s.trim_end_matches(&['<', '(', '{', ';'][..]))
            .map(String::from)
    }

    /// Extract dependencies/imports
    fn extract_dependencies(&self, content: &str) -> Vec<String> {
        let mut deps = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Rust use statements
            if trimmed.starts_with("use ") {
                if let Some(path) = trimmed.strip_prefix("use ") {
                    let path = path.trim_end_matches(';').trim();
                    deps.push(path.to_string());
                }
            }
        }

        deps
    }

    fn get_context(&self, content: &str, line: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let start = line.saturating_sub(2);
        let end = (line + 2).min(lines.len());

        lines[start..end].join("\n")
    }

    /// Search for a symbol
    pub fn find_symbol(&self, name: &str) -> Vec<&SymbolLocation> {
        self.symbols
            .get(name)
            .map(|locs| locs.iter().collect())
            .unwrap_or_default()
    }

    /// Search code by text
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        for (path, file_index) in &self.files {
            for (line_num, line) in file_index.content.lines().enumerate() {
                if line.to_lowercase().contains(&query_lower) {
                    results.push(SearchResult {
                        file: path.clone(),
                        line: line_num + 1,
                        content: line.to_string(),
                        relevance: self.calculate_relevance(line, query),
                    });
                }
            }
        }

        // Sort by relevance
        results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());

        results
    }

    fn calculate_relevance(&self, line: &str, query: &str) -> f64 {
        let line_lower = line.to_lowercase();
        let query_lower = query.to_lowercase();

        // Exact match gets highest score
        if line_lower.contains(&query_lower) {
            1.0
        } else {
            0.5
        }
    }

    /// Get all files
    pub fn files(&self) -> Vec<&PathBuf> {
        self.files.keys().collect()
    }

    /// Get file content
    pub fn get_file_content(&self, path: &Path) -> Option<&str> {
        self.files.get(path).map(|f| f.content.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file: PathBuf,
    pub line: usize,
    pub content: String,
    pub relevance: f64,
}

impl Default for CodebaseIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_extraction() {
        let code = r#"
            fn main() {}
            pub struct MyStruct {}
            pub enum MyEnum {}
        "#;

        let index = CodebaseIndex::new();
        let symbols = index.extract_symbols(code);

        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].name, "main");
        assert_eq!(symbols[1].name, "MyStruct");
    }

    #[test]
    fn test_dependency_extraction() {
        let code = r#"
            use std::collections::HashMap;
            use serde::{Serialize, Deserialize};
        "#;

        let index = CodebaseIndex::new();
        let deps = index.extract_dependencies(code);

        assert_eq!(deps.len(), 2);
    }
}
