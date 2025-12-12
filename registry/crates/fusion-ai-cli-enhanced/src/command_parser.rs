//! Smart command parsing with AI assistance

use crate::{CliError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A parsed command with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    /// The command name
    pub command: String,

    /// Command arguments
    pub args: Vec<String>,

    /// Named flags
    pub flags: HashMap<String, String>,

    /// Boolean flags
    pub bool_flags: Vec<String>,

    /// Original input
    pub original: String,

    /// Parsing confidence (0.0 - 1.0)
    pub confidence: f64,

    /// Command intent
    pub intent: CommandIntent,
}

/// The intent behind a command
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandIntent {
    /// Build or compile
    Build,

    /// Run or execute
    Run,

    /// Test
    Test,

    /// Install dependencies
    Install,

    /// Update/upgrade
    Update,

    /// Clean/remove
    Clean,

    /// Query/search
    Query,

    /// Configuration
    Configure,

    /// Help/documentation
    Help,

    /// Unknown intent
    Unknown,
}

/// Natural language command representation
#[derive(Debug, Clone)]
pub struct NLCommand {
    pub action: String,
    pub target: Option<String>,
    pub modifiers: Vec<String>,
    pub confidence: f64,
}

/// Smart command parser
pub struct CommandParser {
    /// Known command patterns
    patterns: Vec<CommandPattern>,
}

#[derive(Debug, Clone)]
struct CommandPattern {
    name: String,
    aliases: Vec<String>,
    intent: CommandIntent,
    signature: String,
}

impl CommandParser {
    pub fn new() -> Self {
        let mut parser = Self {
            patterns: Vec::new(),
        };
        parser.load_default_patterns();
        parser
    }

    fn load_default_patterns(&mut self) {
        // Build patterns
        self.patterns.push(CommandPattern {
            name: "build".to_string(),
            aliases: vec!["compile".to_string(), "make".to_string()],
            intent: CommandIntent::Build,
            signature: "build [--release] [--target <target>]".to_string(),
        });

        // Run patterns
        self.patterns.push(CommandPattern {
            name: "run".to_string(),
            aliases: vec!["execute".to_string(), "start".to_string()],
            intent: CommandIntent::Run,
            signature: "run [--release] [-- <args>]".to_string(),
        });

        // Test patterns
        self.patterns.push(CommandPattern {
            name: "test".to_string(),
            aliases: vec!["check".to_string()],
            intent: CommandIntent::Test,
            signature: "test [--all] [--release]".to_string(),
        });

        // Install patterns
        self.patterns.push(CommandPattern {
            name: "install".to_string(),
            aliases: vec!["add".to_string(), "get".to_string()],
            intent: CommandIntent::Install,
            signature: "install <package>".to_string(),
        });

        // Clean patterns
        self.patterns.push(CommandPattern {
            name: "clean".to_string(),
            aliases: vec!["remove".to_string(), "delete".to_string()],
            intent: CommandIntent::Clean,
            signature: "clean [--all]".to_string(),
        });
    }

    /// Parse a command string
    pub fn parse(&self, input: &str) -> Result<ParsedCommand> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(CliError::ParseError("Empty input".to_string()));
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CliError::ParseError("No command found".to_string()));
        }

        let command = parts[0].to_string();
        let intent = self.detect_intent(&command);

        let mut args = Vec::new();
        let mut flags = HashMap::new();
        let mut bool_flags = Vec::new();

        let mut i = 1;
        while i < parts.len() {
            let part = parts[i];

            if part.starts_with("--") {
                // Long flag
                let flag_name = part.trim_start_matches("--");
                if i + 1 < parts.len() && !parts[i + 1].starts_with("-") {
                    // Flag with value
                    flags.insert(flag_name.to_string(), parts[i + 1].to_string());
                    i += 2;
                } else {
                    // Boolean flag
                    bool_flags.push(flag_name.to_string());
                    i += 1;
                }
            } else if part.starts_with("-") {
                // Short flag
                let flag_name = part.trim_start_matches("-");
                if i + 1 < parts.len() && !parts[i + 1].starts_with("-") {
                    flags.insert(flag_name.to_string(), parts[i + 1].to_string());
                    i += 2;
                } else {
                    bool_flags.push(flag_name.to_string());
                    i += 1;
                }
            } else {
                // Argument
                args.push(part.to_string());
                i += 1;
            }
        }

        Ok(ParsedCommand {
            command: command.clone(),
            args,
            flags,
            bool_flags,
            original: input.to_string(),
            confidence: 1.0,
            intent,
        })
    }

    fn detect_intent(&self, command: &str) -> CommandIntent {
        for pattern in &self.patterns {
            if pattern.name == command || pattern.aliases.contains(&command.to_string()) {
                return pattern.intent.clone();
            }
        }
        CommandIntent::Unknown
    }

    /// Convert natural language command to structured command
    pub fn from_nl_command(&self, nl: &NLCommand) -> Result<ParsedCommand> {
        // Map natural language action to command
        let command = match nl.action.to_lowercase().as_str() {
            "build" | "compile" | "make" => "build",
            "run" | "execute" | "start" => "run",
            "test" | "check" | "verify" => "test",
            "install" | "add" | "get" => "install",
            "clean" | "remove" | "delete" => "clean",
            _ => {
                return Err(CliError::ParseError(format!(
                    "Unknown action: {}",
                    nl.action
                )))
            }
        };

        let mut args = Vec::new();
        if let Some(ref target) = nl.target {
            args.push(target.clone());
        }

        let intent = self.detect_intent(command);

        Ok(ParsedCommand {
            command: command.to_string(),
            args,
            flags: HashMap::new(),
            bool_flags: nl.modifiers.clone(),
            original: format!("{} {}", nl.action, nl.target.as_deref().unwrap_or("")),
            confidence: nl.confidence,
            intent,
        })
    }

    /// Add a custom command pattern
    pub fn add_pattern(&mut self, name: String, aliases: Vec<String>, intent: CommandIntent) {
        self.patterns.push(CommandPattern {
            name,
            aliases,
            intent,
            signature: "custom".to_string(),
        });
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let parser = CommandParser::new();
        let result = parser.parse("build");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.command, "build");
        assert_eq!(cmd.intent, CommandIntent::Build);
    }

    #[test]
    fn test_parse_with_flags() {
        let parser = CommandParser::new();
        let result = parser.parse("build --release --target x86_64");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert!(cmd.bool_flags.contains(&"release".to_string()));
        assert_eq!(cmd.flags.get("target"), Some(&"x86_64".to_string()));
    }

    #[test]
    fn test_nl_command_conversion() {
        let parser = CommandParser::new();
        let nl = NLCommand {
            action: "build".to_string(),
            target: Some("my-project".to_string()),
            modifiers: vec!["release".to_string()],
            confidence: 0.9,
        };

        let result = parser.from_nl_command(&nl);
        assert!(result.is_ok());
    }
}
