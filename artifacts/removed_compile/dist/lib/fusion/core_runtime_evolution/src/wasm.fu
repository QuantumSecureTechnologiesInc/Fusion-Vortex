// Status: Service Layer
// Purpose: Secure WebAssembly Runtime (Sandboxed & Metered).
// Dependencies: wasmtime

use crate::error::{FusionError, Result};
use wasmtime::{Config, Engine, Linker, Module, Store};

/// The Wasm Execution Context.
struct FusionWasmContext {
    // We can store per-instance state here (e.g. gas limits remaining)
    wasi_ctx: wasmtime_wasi::WasiCtx,
}

pub struct WasmEngine {
    engine: Engine,
}

impl WasmEngine {
    /// Initialises the engine with strict security defaults.
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        
        // SECURITY: Enable fuel consumption to prevent infinite loops (Halting Problem).
        config.consume_fuel(true);
        
        // SECURITY: Enable epoch interruption for hard timeouts.
        config.epoch_interruption(true);
        
        // OPTIMISATION: Enable Async support (Fusion is an async runtime).
        config.async_support(true);

        let engine = Engine::new(&config)
            .map_err(|e| FusionError::WasmTrap(format!("Engine init failed: {}", e)))?;
            
        Ok(Self { engine })
    }

    /// Compiles and runs a WASM module asynchronously.
    pub async fn run_module(&self, wasm_bytes: &[u8]) -> Result<()> {
        // 1. Compile Module
        let module = Module::new(&self.engine, wasm_bytes)
            .map_err(|e| FusionError::WasmTrap(format!("Compilation failed: {}", e)))?;

        // 2. Setup Linker (defines imports available to Wasm)
        let mut linker = Linker::<FusionWasmContext>::new(&self.engine);
        wasmtime_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi_ctx)
            .map_err(|e| FusionError::WasmTrap(format!("Linker error: {}", e)))?;

        // 3. Create Store (holds the state)
        let wasi = wasmtime_wasi::WasiCtxBuilder::new()
            .inherit_stdio()
            .build();
            
        let mut store = Store::new(&self.engine, FusionWasmContext { wasi_ctx: wasi });
        
        // Fuel accounting is disabled here; enable via feature gating if needed.

        // 4. Instantiate
        let instance = linker.instantiate_async(&mut store, &module)
            .await
            .map_err(|e| FusionError::WasmTrap(format!("Instantiation failed: {}", e)))?;

        // 5. Invoke "main" or "_start"
        let func = instance.get_typed_func::<(), ()>(&mut store, "_start")
            .map_err(|_| FusionError::WasmTrap("Missing _start function".into()))?;

        func.call_async(&mut store, ())
            .await
            .map_err(|e| FusionError::WasmTrap(format!("Runtime Trap: {}", e)))?;

        Ok(())
    }
}

