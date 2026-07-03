use regex::Regex;
use super::guardrails::error::SecurityViolation;

pub struct NeuralSanitiser {
    compiled_blocklist_regex: Regex,
}

impl NeuralSanitiser {
    pub fn new() -> Self {
        Self {
            compiled_blocklist_regex: Regex::new(r"(?i)(<script>|javascript:|execute_vortex|drop_tables)")
                .expect("Static regular expression infrastructure breakdown"),
        }
    }

    pub fn cleanse_untrusted_text(&self, raw_input: &str) -> Result<String, SecurityViolation> {
        // Enforce basic text validation against injection patterns
        let normalized = raw_input.replace('\0', "");
        
        if self.compiled_blocklist_regex.is_match(&normalized) {
            return Err(SecurityViolation::ProcessingError);
        }

        Ok(normalized)
    }
}