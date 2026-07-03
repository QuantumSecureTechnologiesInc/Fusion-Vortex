/// Production Trie Search.
///
/// Used to constrain beam search or implement grammar-based decoding.
use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    children: HashMap<u32, TrieNode>, // Token ID -> Child Node
    is_terminal: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_terminal: false,
        }
    }
}

pub struct DecodingTrie {
    root: TrieNode,
}

impl DecodingTrie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /// Adds a constraint sequence to the trie.
    pub fn insert_sequence(&mut self, sequence: &[u32]) {
        let mut current = &mut self.root;
        for &token in sequence {
            current = current.children.entry(token).or_insert_with(TrieNode::new);
        }
        current.is_terminal = true;
    }

    /// Returns a mask (Vec<bool>) that filters logits to only valid next tokens.
    pub fn get_valid_tokens(&self, current_prefix: &[u32]) -> Vec<u32> {
        let mut node = &self.root;

        // Traverse to the current prefix position
        for &token in current_prefix {
            if let Some(child) = node.children.get(&token) {
                node = child;
            } else {
                // Prefix is already invalid, no valid next tokens from this branch
                return Vec::new();
            }
        }

        // Return all possible next tokens (keys of current node's children)
        node.children.keys().cloned().collect()
    }
}
