// src/host_functions.rs
// Host functions callable from WASM plugins

use crate::error::{FusionError, Result};
use std::sync::Arc;
#[cfg(feature = "wasm")]
use wasmtime::{Caller, Linker};

#[cfg(feature = "wasm")]
pub struct HostState {
    pub runtime_handle: crate::executor::RuntimeHandle,
    pub shared_memory: Arc<crate::shared_memory::SharedMemoryManager>,
}

#[cfg(feature = "wasm")]
pub fn register_host_functions(linker: &mut Linker<HostState>) -> Result<()> {
    // Logging
    linker
        .func_wrap(
            "env",
            "host_log",
            |_caller: Caller<'_, HostState>, level: i32, msg_ptr: i32, msg_len: i32| {
                log::info!(
                    "[WASM Plugin] Log level {}: <message at {}:{}>",
                    level,
                    msg_ptr,
                    msg_len
                );
                // In production, we'd read the actual message from WASM memory
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    // File I/O
    linker
        .func_wrap(
            "env",
            "host_read_file",
            |_caller: Caller<'_, HostState>, path_ptr: i32, path_len: i32| -> i32 {
                log::info!("[WASM Plugin] Reading file at {}:{}", path_ptr, path_len);
                // In production: read file and return handle
                101 // Mock file descriptor
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    linker
        .func_wrap(
            "env",
            "host_write_file",
            |_caller: Caller<'_, HostState>, fd: i32, _data_ptr: i32, data_len: i32| -> i32 {
                log::info!("[WASM Plugin] Writing {} bytes to FD {}", data_len, fd);
                data_len // Return bytes written
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    // State management
    linker
        .func_wrap(
            "env",
            "host_get_state",
            |_caller: Caller<'_, HostState>, key_ptr: i32, key_len: i32| -> i32 {
                log::info!(
                    "[WASM Plugin] Getting state for key at {}:{}",
                    key_ptr,
                    key_len
                );
                0 // Mock state value
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    linker
        .func_wrap(
            "env",
            "host_set_state",
            |_caller: Caller<'_, HostState>, key_ptr: i32, key_len: i32, value: i32| {
                log::info!(
                    "[WASM Plugin] Setting state {}:{} = {}",
                    key_ptr,
                    key_len,
                    value
                );
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    // GPU access (NEW!)
    linker
        .func_wrap(
            "env",
            "host_gpu_compute",
            |_caller: Caller<'_, HostState>, device_id: i32, data_ptr: i32, data_len: i32| -> i32 {
                log::info!(
                    "[WASM Plugin] GPU compute on device {} with data {}:{}",
                    device_id,
                    data_ptr,
                    data_len
                );

                // In production: launch GPU kernel with data from WASM memory
                #[cfg(feature = "gpu")]
                {
                    // Access runtime handle from caller state
                    // caller.data().runtime_handle.gpu_kernel(device_id as u32, ...);
                }

                0 // Success
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    // QPU access (NEW!)
    linker
        .func_wrap(
            "env",
            "host_qpu_execute",
            |_caller: Caller<'_, HostState>,
             device_id: i32,
             circuit_ptr: i32,
             circuit_len: i32|
             -> i32 {
                log::info!(
                    "[WASM Plugin] QPU execute on device {} with circuit {}:{}",
                    device_id,
                    circuit_ptr,
                    circuit_len
                );

                // In production: execute quantum circuit
                // caller.data().runtime_handle.qpu_circuit(device_id as u32, ...);

                0 // Success
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    // Shared memory access (NEW!)
    linker
        .func_wrap(
            "env",
            "host_shared_memory",
            |_caller: Caller<'_, HostState>, name_ptr: i32, name_len: i32| -> i32 {
                log::info!(
                    "[WASM Plugin] Accessing shared memory at {}:{}",
                    name_ptr,
                    name_len
                );

                // In production: return pointer to shared memory region
                // let state = caller.data();
                // state.shared_memory.get_tensor(...);

                0 // Mock pointer
            },
        )
        .map_err(|e| FusionError::WasmTrap(e.to_string()))?;

    Ok(())
}

// Stub for when WASM is not enabled
#[cfg(not(feature = "wasm"))]
pub struct HostState;
