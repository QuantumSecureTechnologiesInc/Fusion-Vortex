use serde::{Deserialize, Serialize};

/// Prompt template manager
#[derive(Debug, Clone)]
pub struct PromptManager {
    templates: Vec<PromptTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub variables: Vec<String>,
}

impl PromptManager {
    pub fn new() -> Self {
        Self {
            templates: Self::default_templates(),
        }
    }

    fn default_templates() -> Vec<PromptTemplate> {
        vec![
            PromptTemplate {
                name: "generate".to_string(),
                template: "Generate Fusion code for: {{description}}\n\nContext:\n{{context}}"
                    .to_string(),
                variables: vec!["description".to_string(), "context".to_string()],
            },
            PromptTemplate {
                name: "refactor".to_string(),
                template: "Refactor this Fusion code:\n{{code}}\n\nTo: {{description}}".to_string(),
                variables: vec!["code".to_string(), "description".to_string()],
            },
            PromptTemplate {
                name: "explain".to_string(),
                template: "Explain this Fusion code in detail:\n{{code}}".to_string(),
                variables: vec!["code".to_string()],
            },
        ]
    }

    pub fn render(
        &self,
        template_name: &str,
        variables: &std::collections::HashMap<String, String>,
    ) -> Option<String> {
        let template = self.templates.iter().find(|t| t.name == template_name)?;

        let mut result = template.template.clone();
        for (key, value) in variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }

        Some(result)
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_render_template() {
        let manager = PromptManager::new();
        let mut vars = HashMap::new();
        vars.insert("code".to_string(), "fn test() {}".to_string());

        let result = manager.render("explain", &vars);
        assert!(result.is_some());
        assert!(result.unwrap().contains("fn test() {}"));
    }
}
