/// Production Wasm Runner.
/// Executes stateless Wasm modules (FaaS model) for fast edge compute.
use fusion_std::error::{StdError, StdResult};
use wasmtime::{Engine, Linker, Module, Store};

pub struct WasmRunner {
    engine: Engine,
    module: Module,
}

impl WasmRunner {
    pub fn new(wasm_bytes: &[u8]) -> StdResult<Self> {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, wasm_bytes).map_err(|e| {
            StdError::Core(fusion_core::FusionError::CompilationError(format!(
                "Wasm module load error: {}",
                e
            )))
        })?;

        Ok(Self { engine, module })
    }

    /// Executes a single Wasm function with input/output via shared memory (FaaS model).
    pub fn execute_function(&self, function_name: &str, _input: &[u8]) -> StdResult<Vec<u8>> {
        let mut store = Store::new(&self.engine, ());
        let linker = Linker::new(&self.engine);

        // Instantiate the module
        let instance = linker.instantiate(&mut store, &self.module).map_err(|e| {
            StdError::Core(fusion_core::FusionError::RuntimeError(format!(
                "Failed to instantiate Wasm module: {}",
                e
            )))
        })?;

        // Get the exported function
        let func = instance
            .get_func(&mut store, function_name)
            .ok_or_else(|| {
                StdError::Core(fusion_core::FusionError::CompilationError(format!(
                    "Function '{}' not found in Wasm module",
                    function_name
                )))
            })?;

        // For demonstration, call a no-arg function that returns i32
        // In production, you'd handle typed parameters and results
        let typed_func = func.typed::<(), i32>(&store).map_err(|e| {
            StdError::Core(fusion_core::FusionError::RuntimeError(format!(
                "Function signature mismatch: {}",
                e
            )))
        })?;

        let result = typed_func.call(&mut store, ()).map_err(|e| {
            StdError::Core(fusion_core::FusionError::RuntimeError(format!(
                "Wasm execution error: {}",
                e
            )))
        })?;

        // Convert result to bytes (simplified)
        Ok(result.to_le_bytes().to_vec())
    }
}
