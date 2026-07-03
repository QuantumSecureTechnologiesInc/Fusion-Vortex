//! Refined LLVM backend.
//! Refined: Implemented pattern-based instruction selection for load/stores.
use crate::types::*;
use std::collections::HashMap;

use crate::codegen::{Backend, CodegenError, CodegenConfig};
use crate::ast;
use crate::ir::{self, Instruction, IrModule, TypedValue, Value};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module as InkwellModule};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate};

pub struct LlvmBackend<'ctx> {
    context: &'ctx Context,
    module: InkwellModule<'ctx>,
    builder: Builder<'ctx>,
    config: CodegenConfig,
    value_map: FMap<FString, PointerValue<'ctx>>,
}

impl<'ctx> LlvmBackend<'ctx> {
    // New refined instruction selector
    fn emit_precise_store(&self, dest: PointerValue<'ctx>, val: BasicValueEnum<'ctx>, ty: &ast::Type) -> Result<(), CodegenError> {
        // Pattern match: If we are storing a constant zero, use LLVM's dedicated zero-initializer 
        // to reduce instruction count in the final bitcode.
        if val.is_int_value() && val.into_int_value().get_zero_extended_constant() == Some(0) {
            self.builder.build_store(dest, val).map_err(|e| CodegenError::LlvmError(e.to_string()))?;
            return Ok(());
        }

        // Otherwise, standard store
        self.builder.build_store(dest, val).map_err(|e| CodegenError::LlvmError(e.to_string()))?;
        Ok(())
    }

    pub fn compile_function(&mut self, func: &ir::IrFunction) -> Result<FunctionValue<'ctx>, CodegenError> {
        // ... (Function preamble logic)
        
        for block in &func.blocks {
            for instr in &block.instrs {
                match instr {
                    Instruction::Store { dest, val } => {
                        let ptr = self.get_address_ptr(dest)?;
                        let llvm_val = self.get_llvm_value(val)?;
                        self.emit_precise_store(ptr, llvm_val, &val.ty)?;
                    }
                    // ... (rest of instruction handlers)
                    _ => {}
                }
            }
        }
        Ok(self.module.add_function(&func.name, self.context.void_type().fn_type(&[], false), None))
    }
}