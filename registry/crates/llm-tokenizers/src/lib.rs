/// LLM Tokenizers - BPE, WordPiece, SentencePiece implementations
use fusion_core::{FusionError, FusionResult};
use std::collections::{HashMap, HashSet};

pub mod bpe {
    use super::*;

    /// Byte-Pair Encoding tokenizer
    pub struct BPETokenizer {
        /// Map from token string to ID
        vocab: HashMap<String, usize>,
        /// Map from ID to token string
        id_to_token: HashMap<usize, String>,
        /// Merge rules: pair of tokens -> merged token
        merges: HashMap<(String, String), String>,
        /// Cache for encoded words
        cache: HashMap<String, Vec<usize>>,
    }

    impl BPETokenizer {
        pub fn new(vocab_data: HashMap<String, usize>, merges_list: Vec<(String, String)>) -> Self {
            let mut id_to_token = HashMap::new();
            for (k, v) in &vocab_data {
                id_to_token.insert(*v, k.clone());
            }

            let mut merges = HashMap::new();
            for (p1, p2) in merges_list {
                let combined = format!("{}{}", p1, p2);
                merges.insert((p1, p2), combined);
            }

            Self {
                vocab: vocab_data,
                id_to_token,
                merges,
                cache: HashMap::new(),
            }
        }

        pub fn encode(&self, text: &str) -> Vec<usize> {
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut tokens = Vec::new();

            for word in words {
                if let Some(cached) = self.cache.get(word) {
                    tokens.extend(cached);
                    continue;
                }

                // Initial split into characters (bytes for BPE usually, but chars here for simplicity)
                let mut word_tokens: Vec<String> = word.chars().map(|c| c.to_string()).collect();

                // Iteratively merge
                loop {
                    let mut min_pair_idx = None;
                    let mut best_pair = None;

                    // Find all pairs
                    for i in 0..word_tokens.len().saturating_sub(1) {
                        let pair = (word_tokens[i].clone(), word_tokens[i + 1].clone());
                        if self.merges.contains_key(&pair) {
                            // In real BPE, we'd look up rank. Here we just take the first merge found or iterate.
                            // For simplicity, we merge immediately if found in rule set (greedy).
                            min_pair_idx = Some(i);
                            best_pair = Some(pair);
                            break;
                        }
                    }

                    if let Some(idx) = min_pair_idx {
                        let pair = best_pair.unwrap();
                        let merged = self.merges.get(&pair).unwrap().clone();

                        // Replace pair with merged token
                        word_tokens[idx] = merged;
                        word_tokens.remove(idx + 1);
                    } else {
                        break;
                    }
                }

                // Convert to IDs
                for t in word_tokens {
                    if let Some(id) = self.vocab.get(&t) {
                        tokens.push(*id);
                    } else {
                        // Unknown token handling
                        if let Some(unk) = self.vocab.get("<unk>") {
                            tokens.push(*unk);
                        }
                    }
                }
            }
            tokens
        }

        pub fn decode(&self, tokens: &[usize]) -> String {
            let mut text = String::new();
            for (i, &id) in tokens.iter().enumerate() {
                if let Some(token) = self.id_to_token.get(&id) {
                    text.push_str(token);
                    if i < tokens.len() - 1 {
                        text.push(' '); // Naive spacing
                    }
                }
            }
            text.replace("</w>", "") // Remove end-of-word markers if present
        }
    }
}

pub mod wordpiece {
    use super::*;

    /// WordPiece tokenizer (BERT-style)
    pub struct WordPieceTokenizer {
        vocab: HashMap<String, usize>,
        unk_token: String,
        max_input_chars_per_word: usize,
    }

    impl WordPieceTokenizer {
        pub fn new(vocab: HashMap<String, usize>) -> Self {
            Self {
                vocab,
                unk_token: "[UNK]".to_string(),
                max_input_chars_per_word: 100,
            }
        }

        pub fn tokenize(&self, text: &str) -> Vec<String> {
            let mut output_tokens = Vec::new();
            for token in text.split_whitespace() {
                if token.chars().count() > self.max_input_chars_per_word {
                    output_tokens.push(self.unk_token.clone());
                    continue;
                }

                let mut chars: Vec<char> = token.chars().collect();
                let mut start = 0;
                let mut sub_tokens = Vec::new();
                let mut is_bad = false;

                while start < chars.len() {
                    let mut end = chars.len();
                    let mut cur_substr = None;

                    while end > start {
                        let mut substr: String = chars[start..end].iter().collect();
                        if start > 0 {
                            substr = format!("##{}", substr);
                        }

                        if self.vocab.contains_key(&substr) {
                            cur_substr = Some(substr);
                            break;
                        }
                        end -= 1;
                    }

                    if let Some(s) = cur_substr {
                        sub_tokens.push(s);
                        start = end;
                    } else {
                        is_bad = true;
                        break;
                    }
                }

                if is_bad {
                    output_tokens.push(self.unk_token.clone());
                } else {
                    output_tokens.extend(sub_tokens);
                }
            }
            output_tokens
        }

        pub fn convert_tokens_to_ids(&self, tokens: &[String]) -> Vec<usize> {
            tokens
                .iter()
                .map(|t| {
                    *self
                        .vocab
                        .get(t)
                        .unwrap_or(self.vocab.get("[UNK]").unwrap_or(&0))
                })
                .collect()
        }
    }
}

pub mod sentencepiece {
    use super::*;

    /// SentencePiece tokenizer
    pub struct SentencePieceTokenizer {
        vocab_size: usize,
    }

    impl SentencePieceTokenizer {
        pub fn new(vocab_size: usize) -> Self {
            Self { vocab_size }
        }

        pub fn encode_as_ids(&self, text: &str) -> Vec<usize> {
            // Stub implementation
            text.chars()
                .map(|c| (c as usize) % self.vocab_size)
                .collect()
        }

        pub fn decode_ids(&self, ids: &[usize]) -> String {
            ids.iter().map(|_| '?').collect()
        }
    }
}

