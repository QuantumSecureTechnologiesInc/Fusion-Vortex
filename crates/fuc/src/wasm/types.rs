// src/wasm/types.rs - WASM Type Mappings

use crate::ir::Type;
use wasm_encoder::ValType;

/// Convert Fusion types to WASM types
pub fn fusion_to_wasm_type(fusion_type: &Type) -> Option<ValType> {
    match fusion_type {
        Type::Int => Some(ValType::I64),
        Type::Float => Some(ValType::F64),
        Type::Bool => Some(ValType::I32), // 0 = false, 1 = true
        Type::String => Some(ValType::I32),  // Pointer to memory
        Type::Void => None,                  // No return value
        Type::Struct(_) => Some(ValType::I32), // Heap pointer
        Type::GenericParam(_) => Some(ValType::I32), // Generic resolved to pointer
        Type::Array(_, _) => Some(ValType::I32), // Pointer to array
        Type::Optional(_) => Some(ValType::I32), // Pointer to option
        Type::Union(_) => Some(ValType::I32), // Tagged union pointer
        Type::Closure(_, _) => Some(ValType::I32), // Function table index
        Type::GenericInstance(_, _) => Some(ValType::I32), // Instance pointer
        Type::Unknown => None,               // Should not reach codegen
        Type::Pointer(_) | Type::Slice(_) => Some(ValType::I32),
    }
}

/// Check if a type needs memory allocation
#[allow(dead_code)]
pub fn needs_heap_allocation(fusion_type: &Type) -> bool {
    matches!(
        fusion_type,
        Type::String
            | Type::Struct(_)
            | Type::Array(_, _)
            | Type::Optional(_)
            | Type::Union(_)
            | Type::GenericInstance(_, _)
    )
}

/// Get the size in bytes for a type (for memory allocation)
#[allow(dead_code)]
pub fn type_size_bytes(fusion_type: &Type) -> u32 {
    match fusion_type {
        Type::Int => 8,                // i64
        Type::Float => 8,                  // f64
        Type::Bool => 4,                // i32
        Type::String => 4,                 // pointer
        Type::Struct(_) => 4,              // pointer
        Type::Array(_, _) => 4,               // pointer
        Type::Optional(_) => 4,            // pointer
        Type::Union(_) => 4,               // pointer
        Type::Closure(_, _) => 4,        // function index
        Type::GenericInstance(_, _) => 4, // pointer
        Type::Void => 0,
        Type::GenericParam(_) => 4, // pointer
        Type::Unknown => 0,
        Type::Pointer(_) | Type::Slice(_) => 4,
    }
}