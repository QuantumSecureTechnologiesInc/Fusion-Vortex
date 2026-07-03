use anyhow::Result;
use std::time::Instant;
use wasmtime::*;

pub struct WasmEngine {
    engine: Engine,
    module_cache: std::sync::RwLock<std::collections::HashMap<String, Module>>, // Simple cache by binary content hash would be better, but name is easier for now
}

impl WasmEngine {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.consume_fuel(true); // Prevent infinite loops
                                   // Enable epoch interruption for reliable timeouts
        config.epoch_interruption(true);

        let engine = Engine::new(&config)?;
        Ok(Self {
            engine,
            module_cache: std::sync::RwLock::new(std::collections::HashMap::new()),
        })
    }

    /// Executes a WASM binary.
    /// Uses caching to avoid re-compiling the same plugin.
    pub fn execute(
        &self,
        plugin_name: &str,
        wasm_bytes: &[u8],
        _input: &str,
    ) -> Result<(i32, String, f64)> {
        let start = Instant::now();

        let module = {
            // Try to get from cache first
            let cache = self.module_cache.read().unwrap();
            if let Some(m) = cache.get(plugin_name) {
                m.clone()
            } else {
                drop(cache); // Release read lock
                             // Compile
                let m = Module::from_binary(&self.engine, wasm_bytes)?;
                // Insert into cache
                let mut cache = self.module_cache.write().unwrap();
                cache.insert(plugin_name.to_string(), m.clone());
                m
            }
        };

        let mut store = Store::new(&self.engine, ());
        // Add fuel to limit execution resources (Safety)
        store.set_fuel(100_000)?; // Increased fuel for v2.1
        store.set_epoch_deadline(1);

        // Link nothing for now as v2 definition is simple
        let instance = Instance::new(&mut store, &module, &[])?;

        // Look for the standard entry point "fusion_entry"
        let run_fn = instance.get_typed_func::<(), i32>(&mut store, "fusion_entry")?;

        // Execute
        let exit_code = match run_fn.call(&mut store, ()) {
            Ok(code) => code,
            Err(e) => {
                return Ok((
                    -1,
                    format!("Runtime Error: {}", e),
                    start.elapsed().as_secs_f64() * 1000.0,
                ));
            }
        };

        // In a real scenario, we would read memory here to get string output.
        // For v2.0 MVP, we return a static success message if exit_code == 0.
        let output = if exit_code == 0 {
            format!(
                "Plugin '{}' executed successfully (v2.1 Enhanced).",
                plugin_name
            )
        } else {
            format!("Plugin '{}' reported failure.", plugin_name)
        };

        let duration = start.elapsed().as_secs_f64() * 1000.0;
        Ok((exit_code, output, duration))
    }
}
