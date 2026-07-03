//! Runtime Type Information (Reflection) Generator
//! Addresses: No runtime reflection.
//!
//! Generates metadata tables for structs and functions so that
//! `gc.fu` and user code can query type layouts dynamically at runtime.
use crate::types::*;
use std::collections::HashMap;

use crate::sema::TypedProgram;
use crate::ast::Type;

pub struct FieldMeta {
    pub name: FString,
    pub offset: FSize,
    pub type_id: FSize,
}

pub struct StructMeta {
    pub type_id: FSize,
    pub name: FString,
    pub size: FSize,
    pub fields: FVec<FieldMeta>,
}

pub struct RttiBuilder {
    pub types: FMap<FString, StructMeta>,
    next_type_id: FSize,
}

impl RttiBuilder {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            next_type_id: 100, // Reserve 0-99 for primitive types
        }
    }

    /// Extracts semantic structs and builds a runtime queryable table
    pub fn build_rtti(&mut self, prog: &TypedProgram) {
        for s in &prog.structs {
            let mut fields = Vec::new();
            let mut current_offset = 0;

            for (field_name, field_ty) in &s.fields {
                let size = self.get_type_size(field_ty);
                let t_id = self.get_or_create_type_id(field_ty);

                fields.push(FieldMeta {
                    name: field_name.clone(),
                    offset: current_offset,
                    type_id: t_id,
                });
                
                // Stub: Alignments should be factored in here
                current_offset += size; 
            }

            self.types.insert(s.name.clone(), StructMeta {
                type_id: self.next_type_id,
                name: s.name.clone(),
                size: current_offset,
                fields,
            });
            self.next_type_id += 1;
        }
    }

    fn get_type_size(&self, ty: &Type) -> FSize {
        match ty {
            Type::Int | Type::Bool => 4,
            Type::String | Type::Pointer(_) | Type::Slice(_) => 8,
            Type::Array(inner, count) => self.get_type_size(inner) * count,
            _ => 8, // Default to pointer size for unknown/struct refs
        }
    }

    fn get_or_create_type_id(&mut self, ty: &Type) -> FSize {
        // Native primitives mapping
        match ty {
            Type::Int => 1,
            Type::Bool => 2,
            Type::String => 3,
            _ => 99, // Dynamic
        }
    }
}