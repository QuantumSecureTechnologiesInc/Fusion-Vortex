// src/docs/search_index.rs - Search Index Builder
#![allow(dead_code)]
// Creates searchable index of documentation

use super::{DocItem, DocItemType};
use std::collections::HashMap;

/// Search index entry
#[derive(Debug, Clone)]
pub struct IndexEntry {
    /// Item name
    pub name: String,
    /// Full qualified name
    pub full_name: String,
    /// Item type
    pub item_type: DocItemType,
    /// Summary (first line of docs)
    pub summary: String,
    /// Full documentation
    pub documentation: String,
    /// URL path
    pub url: String,
    /// Search weight
    pub weight: f32,
}

/// Search index
pub struct SearchIndex {
    entries: Vec<IndexEntry>,
    inverted_index: HashMap<String, Vec<usize>>,
}

impl SearchIndex {
    /// Create a new search index
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            inverted_index: HashMap::new(),
        }
    }

    /// Add items to index
    pub fn add_items(&mut self, items: &[DocItem]) {
        for item in items {
            self.add_item(item);
        }
    }

    /// Add single item to index
    pub fn add_item(&mut self, item: &DocItem) {
        let entry_index = self.entries.len();

        let summary = item.documentation.lines().next().unwrap_or("").to_string();

        let weight = Self::calculate_weight(&item.item_type);

        let entry = IndexEntry {
            name: item.name.clone(),
            full_name: item.full_name(),
            item_type: item.item_type.clone(),
            summary,
            documentation: item.documentation.clone(),
            url: Self::item_to_url(&item.full_name()),
            weight,
        };

        // Index by name words
        for word in Self::tokenize(&item.name) {
            self.inverted_index
                .entry(word)
                .or_insert_with(Vec::new)
                .push(entry_index);
        }

        // Index by documentation words
        for word in Self::tokenize(&item.documentation) {
            self.inverted_index
                .entry(word)
                .or_insert_with(Vec::new)
                .push(entry_index);
        }

        self.entries.push(entry);
    }

    /// Search the index
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let query_words: Vec<String> = Self::tokenize(query);
        let mut scores: HashMap<usize, f32> = HashMap::new();

        // Score each entry based on query words
        for word in &query_words {
            if let Some(entry_indices) = self.inverted_index.get(&word.to_lowercase()) {
                for &idx in entry_indices {
                    let entry = &self.entries[idx];
                    let score = self.calculate_score(entry, word);
                    *scores.entry(idx).or_insert(0.0) += score;
                }
            }
        }

        // Convert to results and sort by score
        let mut results: Vec<SearchResult> = scores
            .into_iter()
            .map(|(idx, score)| {
                let entry = &self.entries[idx];
                SearchResult {
                    name: entry.name.clone(),
                    full_name: entry.full_name.clone(),
                    item_type: entry.item_type.clone(),
                    summary: entry.summary.clone(),
                    url: entry.url.clone(),
                    score,
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }

    /// Calculate search score for an entry
    fn calculate_score(&self, entry: &IndexEntry, query_word: &str) -> f32 {
        let mut score = 0.0;

        // Exact name match
        if entry.name.to_lowercase() == query_word.to_lowercase() {
            score += 10.0;
        }
        // Name starts with query
        else if entry
            .name
            .to_lowercase()
            .starts_with(&query_word.to_lowercase())
        {
            score += 5.0;
        }
        // Name contains query
        else if entry
            .name
            .to_lowercase()
            .contains(&query_word.to_lowercase())
        {
            score += 2.0;
        }

        // Documentation contains query
        if entry
            .documentation
            .to_lowercase()
            .contains(&query_word.to_lowercase())
        {
            score += 0.5;
        }

        // Apply weight
        score * entry.weight
    }

    /// Calculate weight for item type
    fn calculate_weight(item_type: &DocItemType) -> f32 {
        match item_type {
            DocItemType::Function => 1.0,
            DocItemType::Class => 1.2,
            DocItemType::Trait => 1.1,
            DocItemType::Module => 0.8,
            DocItemType::Constant => 0.6,
            DocItemType::TypeAlias => 0.7,
        }
    }

    /// Tokenize text into words
    fn tokenize(text: &str) -> Vec<String> {
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }

    /// Convert item name to URL
    fn item_to_url(name: &str) -> String {
        format!("/{}.html", name.replace("::", "/"))
    }

    /// Export index as JSON
    pub fn to_json(&self) -> String {
        // In production, use serde_json
        let mut json = String::from("[\n");

        for (i, entry) in self.entries.iter().enumerate() {
            json.push_str(&format!(
                r#"  {{"name": "{}", "type": "{:?}", "summary": "{}", "url": "{}"}}"#,
                entry.name,
                entry.item_type,
                entry.summary.replace('"', "\\\""),
                entry.url
            ));
            if i < self.entries.len() - 1 {
                json.push(',');
            }
            json.push('\n');
        }

        json.push_str("]\n");
        json
    }

    /// Get statistics
    pub fn stats(&self) -> IndexStats {
        IndexStats {
            total_entries: self.entries.len(),
            total_words: self.inverted_index.len(),
            average_docs_per_word: if self.inverted_index.is_empty() {
                0.0
            } else {
                self.inverted_index.values().map(|v| v.len()).sum::<usize>() as f32
                    / self.inverted_index.len() as f32
            },
        }
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub full_name: String,
    pub item_type: DocItemType,
    pub summary: String,
    pub url: String,
    pub score: f32,
}

/// Index statistics
#[derive(Debug)]
pub struct IndexStats {
    pub total_entries: usize,
    pub total_words: usize,
    pub average_docs_per_word: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_index_creation() {
        let index = SearchIndex::new();
        assert_eq!(index.entries.len(), 0);
    }

    #[test]
    fn test_tokenize() {
        let words = SearchIndex::tokenize("Hello, world! This is a test.");
        assert!(words.contains(&"hello".to_string()));
        assert!(words.contains(&"world".to_string()));
        assert!(words.contains(&"test".to_string()));
    }

    #[test]
    fn test_add_and_search() {
        let mut index = SearchIndex::new();

        let item = DocItem::new(DocItemType::Function, "test_function", "fn test_function()")
            .with_docs("This function performs a test");

        index.add_item(&item);

        let results = index.search("test");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "test_function");
    }

    #[test]
    fn test_calculate_weight() {
        assert_eq!(SearchIndex::calculate_weight(&DocItemType::Class), 1.2);
        assert_eq!(SearchIndex::calculate_weight(&DocItemType::Function), 1.0);
    }

    #[test]
    fn test_item_to_url() {
        assert_eq!(
            SearchIndex::item_to_url("std::vec::Vec"),
            "/std/vec/Vec.html"
        );
    }

    #[test]
    fn test_search_scoring() {
        let mut index = SearchIndex::new();

        // Add multiple items
        index.add_item(&DocItem::new(
            DocItemType::Function,
            "add",
            "fn add(a: int, b: int) -> int",
        ));

        index.add_item(&DocItem::new(
            DocItemType::Function,
            "add_float",
            "fn add_float(a: float, b: float) -> float",
        ));

        let results = index.search("add");

        // Exact match should score higher
        assert_eq!(results[0].name, "add");
        assert!(results[0].score > results[1].score);
    }
}
