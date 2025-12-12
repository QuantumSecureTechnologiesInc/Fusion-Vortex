//! Intelligent command suggestions

use crate::{CliError, Result};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};

/// A command suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSuggestion {
    /// The suggested command
    pub command: String,

    /// Description of what it does
    pub description: String,

    /// Relevance score (0.0 - 1.0)
    pub relevance: f64,

    /// Usage example
    pub example: Option<String>,

    /// Why this is suggested
    pub reasoning: String,
}

/// Context for generating suggestions
#[derive(Debug, Clone)]
pub struct SuggestionContext {
    pub partial_input: String,
    pub history: Vec<String>,
    pub current_directory: String,
}

/// Engine for generating intelligent suggestions
pub struct SuggestionEngine {
    known_commands: Vec<KnownCommand>,
    matcher: SkimMatcherV2,
}

#[derive(Debug, Clone)]
struct KnownCommand {
    name: String,
    description: String,
    examples: Vec<String>,
    aliases: Vec<String>,
    category: CommandCategory,
}

#[derive(Debug, Clone, PartialEq)]
enum CommandCategory {
    Build,
    Run,
    Test,
    Package,
    Development,
    Configuration,
    Help,
}

impl SuggestionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            known_commands: Vec::new(),
            matcher: SkimMatcherV2::default(),
        };
        engine.load_known_commands();
        engine
    }

    fn load_known_commands(&mut self) {
        // Build commands
        self.known_commands.push(KnownCommand {
            name: "build".to_string(),
            description: "Build the project".to_string(),
            examples: vec![
                "build --release".to_string(),
                "build --target wasm32-unknown-unknown".to_string(),
            ],
            aliases: vec!["compile".to_string(), "make".to_string()],
            category: CommandCategory::Build,
        });

        // Run commands
        self.known_commands.push(KnownCommand {
            name: "run".to_string(),
            description: "Run the project".to_string(),
            examples: vec!["run --release".to_string(), "run -- arg1 arg2".to_string()],
            aliases: vec!["execute".to_string(), "start".to_string()],
            category: CommandCategory::Run,
        });

        // Test commands
        self.known_commands.push(KnownCommand {
            name: "test".to_string(),
            description: "Run tests".to_string(),
            examples: vec![
                "test --all".to_string(),
                "test integration_tests".to_string(),
            ],
            aliases: vec!["check".to_string()],
            category: CommandCategory::Test,
        });

        // Package commands
        self.known_commands.push(KnownCommand {
            name: "package".to_string(),
            description: "Create a distributable package".to_string(),
            examples: vec!["package --format zip".to_string()],
            aliases: vec!["pack".to_string()],
            category: CommandCategory::Package,
        });

        // Development commands
        self.known_commands.push(KnownCommand {
            name: "watch".to_string(),
            description: "Watch for changes and rebuild".to_string(),
            examples: vec!["watch --clear".to_string()],
            aliases: vec!["dev".to_string()],
            category: CommandCategory::Development,
        });

        // Configuration commands
        self.known_commands.push(KnownCommand {
            name: "config".to_string(),
            description: "Configure project settings".to_string(),
            examples: vec!["config set key value".to_string()],
            aliases: vec!["configure".to_string(), "settings".to_string()],
            category: CommandCategory::Configuration,
        });

        // Help commands
        self.known_commands.push(KnownCommand {
            name: "help".to_string(),
            description: "Show help information".to_string(),
            examples: vec!["help build".to_string()],
            aliases: vec!["--help".to_string(), "-h".to_string()],
            category: CommandCategory::Help,
        });
    }

    /// Generate suggestions based on context
    pub fn generate(&self, ctx: &SuggestionContext) -> Result<Vec<CommandSuggestion>> {
        let mut suggestions = Vec::new();

        if ctx.partial_input.is_empty() {
            // No input yet - suggest based on history
            suggestions.extend(self.suggest_from_history(&ctx.history));
        } else {
            // Fuzzy match against known commands
            suggestions.extend(self.fuzzy_match_commands(&ctx.partial_input));
        }

        // Sort by relevance
        suggestions.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());

        // Limit to top 10
        suggestions.truncate(10);

        if suggestions.is_empty() {
            Err(CliError::NoSuggestions(ctx.partial_input.clone()))
        } else {
            Ok(suggestions)
        }
    }

    fn suggest_from_history(&self, history: &[String]) -> Vec<CommandSuggestion> {
        if history.is_empty() {
            // Suggest common starting commands
            return vec![
                CommandSuggestion {
                    command: "build".to_string(),
                    description: "Build the project".to_string(),
                    relevance: 0.9,
                    example: Some("build --release".to_string()),
                    reasoning: "Common first command".to_string(),
                },
                CommandSuggestion {
                    command: "test".to_string(),
                    description: "Run tests".to_string(),
                    relevance: 0.85,
                    example: Some("test --all".to_string()),
                    reasoning: "Verify project setup".to_string(),
                },
            ];
        }

        // Suggest based on last command
        let last = history.last().unwrap();
        let mut suggestions = Vec::new();

        if last.starts_with("build") {
            suggestions.push(CommandSuggestion {
                command: "run".to_string(),
                description: "Run the built project".to_string(),
                relevance: 0.95,
                example: Some("run --release".to_string()),
                reasoning: "Typical next step after building".to_string(),
            });
        } else if last.starts_with("test") {
            suggestions.push(CommandSuggestion {
                command: "build --release".to_string(),
                description: "Build optimised version".to_string(),
                relevance: 0.9,
                example: None,
                reasoning: "Tests passed, ready for release build".to_string(),
            });
        }

        suggestions
    }

    fn fuzzy_match_commands(&self, partial: &str) -> Vec<CommandSuggestion> {
        let mut matches = Vec::new();

        for cmd in &self.known_commands {
            // Check main name
            if let Some(score) = self.matcher.fuzzy_match(&cmd.name, partial) {
                let relevance = (score as f64) / 100.0;
                if relevance > 0.3 {
                    matches.push(CommandSuggestion {
                        command: cmd.name.clone(),
                        description: cmd.description.clone(),
                        relevance,
                        example: cmd.examples.first().cloned(),
                        reasoning: format!("Matches '{}'", partial),
                    });
                }
            }

            // Check aliases
            for alias in &cmd.aliases {
                if let Some(score) = self.matcher.fuzzy_match(alias, partial) {
                    let relevance = (score as f64) / 100.0;
                    if relevance > 0.3 {
                        matches.push(CommandSuggestion {
                            command: cmd.name.clone(),
                            description: format!("{} (alias: {})", cmd.description, alias),
                            relevance,
                            example: cmd.examples.first().cloned(),
                            reasoning: format!("Alias '{}' matches '{}'", alias, partial),
                        });
                    }
                }
            }
        }

        matches
    }

    /// Add a custom command to the suggestion database
    pub fn add_command(&mut self, name: String, description: String, category: String) {
        let cat = match category.as_str() {
            "build" => CommandCategory::Build,
            "run" => CommandCategory::Run,
            "test" => CommandCategory::Test,
            "package" => CommandCategory::Package,
            "dev" => CommandCategory::Development,
            "config" => CommandCategory::Configuration,
            _ => CommandCategory::Help,
        };

        self.known_commands.push(KnownCommand {
            name,
            description,
            examples: Vec::new(),
            aliases: Vec::new(),
            category: cat,
        });
    }
}

impl Default for SuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_generation() {
        let engine = SuggestionEngine::new();
        let ctx = SuggestionContext {
            partial_input: "bui".to_string(),
            history: Vec::new(),
            current_directory: "/project".to_string(),
        };

        let suggestions = engine.generate(&ctx);
        assert!(suggestions.is_ok());
        let sug = suggestions.unwrap();
        assert!(!sug.is_empty());
    }

    #[test]
    fn test_history_suggestions() {
        let engine = SuggestionEngine::new();
        let ctx = SuggestionContext {
            partial_input: String::new(),
            history: vec!["build --release".to_string()],
            current_directory: "/project".to_string(),
        };

        let suggestions = engine.generate(&ctx);
        assert!(suggestions.is_ok());
    }
}
