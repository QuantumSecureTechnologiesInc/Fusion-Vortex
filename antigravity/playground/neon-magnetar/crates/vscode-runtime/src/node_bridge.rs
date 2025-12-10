//! Node.js bridge for executing JavaScript-based VS Code extensions

use anyhow::Result;
use boa_engine::{Context, JsValue, Source};
use std::path::Path;

/// Node.js runtime bridge using Boa JavaScript engine
pub struct NodeBridge {
    context: Context,
}

impl NodeBridge {
    pub fn new() -> Result<Self> {
        let mut context = Context::default();

        // Initialize Node.js-like global objects
        Self::setup_node_globals(&mut context)?;

        Ok(Self { context })
    }

    /// Set up Node.js-compatible global objects
    fn setup_node_globals(context: &mut Context) -> Result<()> {
        // Add require() function stub
        let require_fn = r#"
            function require(module) {
                console.log('Loading module:', module);
                return {};
            }
        "#;
        context.eval(Source::from_bytes(require_fn))?;

        // Add process object
        let process_obj = r#"
            var process = {
                env: {},
                cwd: function() { return '/'; },
                exit: function(code) { console.log('Exit:', code); }
            };
        "#;
        context.eval(Source::from_bytes(process_obj))?;

        // Add console object
        let console_obj = r#"
            var console = {
                log: function(...args) { /* bridge to tracing */ },
                error: function(...args) { /* bridge to tracing */ },
                warn: function(...args) { /* bridge to tracing */ }
            };
        "#;
        context.eval(Source::from_bytes(console_obj))?;

        Ok(())
    }

    /// Execute JavaScript code
    pub fn execute(&mut self, code: &str) -> Result<JsValue> {
        let result = self.context.eval(Source::from_bytes(code))?;
        Ok(result)
    }

    /// Load and execute a JavaScript file
    pub async fn load_module(&mut self, path: &Path) -> Result<JsValue> {
        let code = tokio::fs::read_to_string(path).await?;
        self.execute(&code)
    }

    /// Call a JavaScript function
    pub fn call_function(&mut self, name: &str, args: &[JsValue]) -> Result<JsValue> {
        let function_source = format!("{}(...args)", name);

        // Set args in context
        self.context.register_global_property(
            "args",
            JsValue::from(args.to_vec()),
            boa_engine::property::Attribute::all(),
        )?;

        let result = self.context.eval(Source::from_bytes(&function_source))?;
        Ok(result)
    }
}

/// Convert JS values to Rust types
pub fn js_value_to_json(value: &JsValue) -> Result<serde_json::Value> {
    // Simplified conversion
    if value.is_null() || value.is_undefined() {
        Ok(serde_json::Value::Null)
    } else if value.is_boolean() {
        Ok(serde_json::Value::Bool(value.as_boolean().unwrap()))
    } else if value.is_number() {
        Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(value.as_number().unwrap())
                .unwrap_or(serde_json::Number::from(0)),
        ))
    } else if value.is_string() {
        Ok(serde_json::Value::String(
            value.as_string().unwrap().to_std_string().unwrap(),
        ))
    } else {
        Ok(serde_json::Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_bridge_creation() {
        let bridge = NodeBridge::new();
        assert!(bridge.is_ok());
    }

    #[test]
    fn test_execute_javascript() {
        let mut bridge = NodeBridge::new().unwrap();
        let result = bridge.execute("1 + 1");
        assert!(result.is_ok());
    }
}
