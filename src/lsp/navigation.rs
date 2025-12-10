#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::path::PathBuf;
use tower_lsp::lsp_types::{Location, Position, Range, Url};

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub definition_range: Range,
    pub references: Vec<Location>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Class,
    Struct,
    Trait,
    Variable,
    Constant,
    Module,
    Enum,
    EnumVariant,
}

/// Workspace-wide symbol index
pub struct WorkspaceIndex {
    symbols: HashMap<String, SymbolInfo>,
    file_symbols: HashMap<PathBuf, Vec<String>>,
    module_tree: HashMap<String, Vec<String>>,
}

impl WorkspaceIndex {
    pub fn new() -> Self {
        WorkspaceIndex {
            symbols: HashMap::new(),
            file_symbols: HashMap::new(),
            module_tree: HashMap::new(),
        }
    }

    /// Add a symbol to the index
    pub fn add_symbol(&mut self, symbol: SymbolInfo) {
        let name = symbol.name.clone();
        let file_path = url_to_path(&symbol.location.uri);

        // Add to global symbol table
        self.symbols.insert(name.clone(), symbol);

        // Add to file-specific index
        self.file_symbols
            .entry(file_path)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Find symbol by name
    pub fn find_symbol(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }

    /// Find all symbols in a file
    pub fn symbols_in_file(&self, file: &PathBuf) -> Vec<&SymbolInfo> {
        if let Some(symbol_names) = self.file_symbols.get(file) {
            symbol_names
                .iter()
                .filter_map(|name| self.symbols.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find symbol at position
    pub fn symbol_at_position(&self, uri: &Url, position: Position) -> Option<&SymbolInfo> {
        let file_path = url_to_path(uri);

        for symbol in self.symbols_in_file(&file_path) {
            if position_in_range(position, symbol.definition_range) {
                return Some(symbol);
            }
        }

        None
    }

    /// Find all references to a symbol
    pub fn find_references(&self, symbol_name: &str) -> Vec<Location> {
        if let Some(symbol) = self.symbols.get(symbol_name) {
            let mut refs = symbol.references.clone();
            refs.push(symbol.location.clone());
            refs
        } else {
            Vec::new()
        }
    }

    /// Get symbol definition location
    pub fn get_definition(&self, symbol_name: &str) -> Option<Location> {
        self.symbols.get(symbol_name).map(|s| s.location.clone())
    }

    /// Update symbol references
    pub fn add_reference(&mut self, symbol_name: &str, location: Location) {
        if let Some(symbol) = self.symbols.get_mut(symbol_name) {
            symbol.references.push(location);
        }
    }

    /// Clear all symbols from a file (for re-indexing)
    pub fn clear_file(&mut self, file: &PathBuf) {
        if let Some(symbol_names) = self.file_symbols.remove(file) {
            for name in symbol_names {
                self.symbols.remove(&name);
            }
        }
    }

    /// Get all symbols matching a query (for workspace symbols)
    pub fn search_symbols(&self, query: &str) -> Vec<&SymbolInfo> {
        let query_lower = query.to_lowercase();

        self.symbols
            .values()
            .filter(|symbol| symbol.name.to_lowercase().contains(&query_lower))
            .collect()
    }
}

/// Rename engine for safe symbol renaming
pub struct RenameEngine {
    index: WorkspaceIndex,
}

impl RenameEngine {
    pub fn new(index: WorkspaceIndex) -> Self {
        RenameEngine { index }
    }

    /// Check if rename is safe
    pub fn can_rename(&self, old_name: &str, new_name: &str) -> Result<(), String> {
        // Check if old symbol exists
        if !self.index.symbols.contains_key(old_name) {
            return Err(format!("Symbol '{}' not found", old_name));
        }

        // Check if new name conflicts
        if self.index.symbols.contains_key(new_name) {
            return Err(format!("Symbol '{}' already exists", new_name));
        }

        // Check naming rules
        if !is_valid_identifier(new_name) {
            return Err(format!("'{}' is not a valid identifier", new_name));
        }

        Ok(())
    }

    /// Get all locations that need to be renamed
    pub fn get_rename_locations(&self, symbol_name: &str) -> Vec<Location> {
        self.index.find_references(symbol_name)
    }

    /// Prepare workspace edit for rename
    pub fn prepare_rename_edit(
        &self,
        symbol_name: &str,
        new_name: &str,
    ) -> Result<HashMap<Url, Vec<(Range, String)>>, String> {
        self.can_rename(symbol_name, new_name)?;

        let locations = self.get_rename_locations(symbol_name);
        let mut edits: HashMap<Url, Vec<(Range, String)>> = HashMap::new();

        for location in locations {
            edits
                .entry(location.uri.clone())
                .or_insert_with(Vec::new)
                .push((location.range, new_name.to_string()));
        }

        Ok(edits)
    }
}

/// Code action provider for quick fixes and refactorings
pub struct CodeActionProvider {
    index: WorkspaceIndex,
}

impl CodeActionProvider {
    pub fn new(index: WorkspaceIndex) -> Self {
        CodeActionProvider { index }
    }

    /// Get available code actions at position
    pub fn get_actions(&self, uri: &Url, range: Range) -> Vec<CodeAction> {
        let mut actions = Vec::new();

        // Check if cursor is on a symbol
        let position = range.start;
        if let Some(symbol) = self.index.symbol_at_position(uri, position) {
            // Add "Rename" action
            actions.push(CodeAction {
                title: format!("Rename '{}'", symbol.name),
                kind: CodeActionKind::Refactor,
                command: Some(format!("rename:{}", symbol.name)),
            });

            // Add "Find all references" action
            actions.push(CodeAction {
                title: format!("Find all references to '{}'", symbol.name),
                kind: CodeActionKind::Source,
                command: Some(format!("references:{}", symbol.name)),
            });

            // Add "Go to definition" action
            actions.push(CodeAction {
                title: format!("Go to definition of '{}'", symbol.name),
                kind: CodeActionKind::Source,
                command: Some(format!("definition:{}", symbol.name)),
            });
        }

        // Add generic actions
        actions.push(CodeAction {
            title: "Format document".to_string(),
            kind: CodeActionKind::SourceOrganizeImports,
            command: Some("format".to_string()),
        });

        actions
    }
}

#[derive(Debug, Clone)]
pub struct CodeAction {
    pub title: String,
    pub kind: CodeActionKind,
    pub command: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    Source,
    SourceOrganizeImports,
}

/// Helper functions

fn url_to_path(url: &Url) -> PathBuf {
    PathBuf::from(url.path())
}

fn position_in_range(position: Position, range: Range) -> bool {
    if position.line < range.start.line || position.line > range.end.line {
        return false;
    }

    if position.line == range.start.line && position.character < range.start.character {
        return false;
    }

    if position.line == range.end.line && position.character > range.end.character {
        return false;
    }

    true
}

fn is_valid_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();

    // First character must be letter or underscore
    if let Some(first) = chars.next() {
        if !first.is_alphabetic() && first != '_' {
            return false;
        }
    }

    // Rest can be letters, digits, or underscores
    for c in chars {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifier() {
        assert!(is_valid_identifier("foo"));
        assert!(is_valid_identifier("_bar"));
        assert!(is_valid_identifier("foo123"));
        assert!(is_valid_identifier("FooBar"));

        assert!(!is_valid_identifier("123foo"));
        assert!(!is_valid_identifier("foo-bar"));
        assert!(!is_valid_identifier(""));
    }

    #[test]
    fn test_workspace_index() {
        let mut index = WorkspaceIndex::new();

        let symbol = SymbolInfo {
            name: "test_function".to_string(),
            kind: SymbolKind::Function,
            location: Location {
                uri: Url::parse("file:///test.fu").unwrap(),
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 10,
                    },
                },
            },
            definition_range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 10,
                },
            },
            references: Vec::new(),
        };

        index.add_symbol(symbol);

        assert!(index.find_symbol("test_function").is_some());
        assert!(index.find_symbol("nonexistent").is_none());
    }
}
