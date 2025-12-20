//! WebAssembly runtime for executing compiled VS Code extensions

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;

/// WASM runtime for executing WebAssembly-compiled extensions
pub struct WasmRuntime {
    store: Store,
    instances: HashMap<String, Instance>,
}

impl WasmRuntime {
    pub fn new() -> Result<Self> {
        let compiler = Cranelift::default();
        let store = Store::new(compiler);

        Ok(Self {
            store,
            instances: HashMap::new(),
        })
    }

    /// Load a WASM module from file
    pub async fn load_module(&mut self, path: &Path, module_id: String) -> Result<()> {
        let wasm_bytes = tokio::fs::read(path).await?;
        let module = Module::new(&self.store, &wasm_bytes)?;

        // Create imports for VS Code API
        let import_object = self.create_vscode_imports();

        // Instantiate the module
        let instance = Instance::new(&mut self.store, &module, &import_object)?;

        self.instances.insert(module_id, instance);

        Ok(())
    }

    /// Create imports for VS Code API functions
    fn create_vscode_imports(&mut self) -> wasmer::Imports {
        let env = FunctionEnv::new(&mut self.store, ());

        imports! {
            "vscode" => {
                "window_showInformationMessage" => Function::new_typed_with_env(
                    &mut self.store,
                    &env,
                    Self::window_show_information_message
                ),
                "workspace_findFiles" => Function::new_typed_with_env(
                    &mut self.store,
                    &env,
                    Self::workspace_find_files
                ),
                "commands_registerCommand" => Function::new_typed_with_env(
                    &mut self.store,
                    &env,
                    Self::commands_register_command
                ),
            }
        }
    }

    /// Import: window.showInformationMessage
    fn window_show_information_message(
        _env: FunctionEnvMut<()>,
        message_ptr: i32,
        message_len: i32,
    ) {
        tracing::info!(
            "[WASM Extension] Information: message at ptr {}, len {}",
            message_ptr,
            message_len
        );
        // In real implementation, read string from WASM memory
    }

    /// Import: workspace.findFiles
    fn workspace_find_files(_env: FunctionEnvMut<()>, pattern_ptr: i32, _pattern_len: i32) -> i32 {
        tracing::debug!(
            "[WASM Extension] Find files: pattern at ptr {}",
            pattern_ptr
        );
        // Return pointer to result array
        0
    }

    /// Import: commands.registerCommand
    fn commands_register_command(
        _env: FunctionEnvMut<()>,
        name_ptr: i32,
        _name_len: i32,
        _callback_fn: i32,
    ) -> i32 {
        tracing::debug!("[WASM Extension] Register command at ptr {}", name_ptr);
        1 // Success
    }

    /// Write string to WASM memory
    pub fn write_string(&mut self, module_id: &str, s: &str) -> Result<i32> {
        let bytes = s.as_bytes();
        let ptr = self.allocate_memory(module_id, bytes.len() as i32)?;

        let memory = self.get_memory(module_id)?;
        let view = memory.view(&self.store);
        view.write(ptr as u64, bytes)?;

        Ok(ptr)
    }

    /// Call an exported function from a loaded module
    pub fn call_function(
        &mut self,
        module_id: &str,
        function_name: &str,
        args: &[wasmer::Value],
    ) -> Result<Box<[wasmer::Value]>> {
        let instance = self
            .instances
            .get(module_id)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", module_id))?;

        let function = instance.exports.get_function(function_name)?;
        let result = function.call(&mut self.store, args)?;

        Ok(result)
    }

    /// Activate a WASM extension
    pub fn activate(&mut self, module_id: &str) -> Result<()> {
        tracing::info!("Activating WASM extension: {}", module_id);

        // Call the activate function if it exists
        if let Ok(_) = self.call_function(module_id, "activate", &[]) {
            tracing::info!("Extension {} activated successfully", module_id);
        }

        Ok(())
    }

    /// Deactivate a WASM extension
    pub fn deactivate(&mut self, module_id: &str) -> Result<()> {
        tracing::info!("Deactivating WASM extension: {}", module_id);

        // Call the deactivate function if it exists
        if let Ok(_) = self.call_function(module_id, "deactivate", &[]) {
            tracing::info!("Extension {} deactivated successfully", module_id);
        }

        // Remove from instances
        self.instances.remove(module_id);

        Ok(())
    }

    /// Get memory from a module instance
    pub fn get_memory(&self, module_id: &str) -> Result<wasmer::Memory> {
        let instance = self
            .instances
            .get(module_id)
            .ok_or_else(|| anyhow::anyhow!("Module not found: {}", module_id))?;

        let memory = instance.exports.get_memory("memory")?;
        Ok(memory.clone())
    }

    /// Read string from WASM memory
    pub fn read_string(&self, module_id: &str, ptr: i32, len: i32) -> Result<String> {
        let memory = self.get_memory(module_id)?;
        let view = memory.view(&self.store);

        let mut bytes = vec![0u8; len as usize];
        view.read(ptr as u64, &mut bytes)?;

        Ok(String::from_utf8(bytes)?)
    }

    /// Allocate memory in WASM module
    fn allocate_memory(&mut self, module_id: &str, size: i32) -> Result<i32> {
        // Call malloc or similar function exported by the module
        let result = self.call_function(module_id, "malloc", &[wasmer::Value::I32(size)])?;

        if let Some(wasmer::Value::I32(ptr)) = result.first() {
            Ok(*ptr)
        } else {
            Err(anyhow::anyhow!("Failed to allocate memory"))
        }
    }
}

/// WASM extension metadata
#[derive(Debug, Clone)]
pub struct WasmExtensionMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    pub wasm_path: std::path::PathBuf,
}

impl WasmExtensionMeta {
    pub fn from_manifest(path: &Path) -> Result<Self> {
        let manifest_content = std::fs::read_to_string(path)?;
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)?;

        let id = manifest["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing name in manifest"))?
            .to_string();

        let name = manifest
            .get("displayName")
            .and_then(|n| n.as_str())
            .unwrap_or(&id)
            .to_string();

        let version = manifest["version"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing version in manifest"))?
            .to_string();

        let wasm_file = manifest
            .get("wasm")
            .and_then(|w| w.as_str())
            .unwrap_or("extension.wasm");

        let wasm_path = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid manifest path"))?
            .join(wasm_file);

        Ok(Self {
            id,
            name,
            version,
            wasm_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_runtime_creation() {
        let runtime = WasmRuntime::new();
        assert!(runtime.is_ok());
    }
}
