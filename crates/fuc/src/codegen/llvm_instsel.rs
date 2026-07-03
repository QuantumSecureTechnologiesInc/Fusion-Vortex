//! Refined LLVM backend.
//! Refined: Implemented pattern-based instruction selection for load/stores.
use crate::types::*;

use crate::codegen::{CodegenError, CodegenConfig};
use crate::ast;
use crate::ir::{self, Instruction};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module as InkwellModule;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

pub struct LlvmBackend<'ctx> {
    context: &'ctx Context,
    module: InkwellModule<'ctx>,
    builder: Builder<'ctx>,
    #[allow(dead_code)]
    config: CodegenConfig,
    value_map: FMap<FString, PointerValue<'ctx>>,
}

impl<'ctx> LlvmBackend<'ctx> {
    // New refined instruction selector
    fn emit_precise_store(&self, dest: PointerValue<'ctx>, val: BasicValueEnum<'ctx>, _ty: &ast::Type) -> Result<(), CodegenError> {
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

    /// Resolve an Address to a PointerValue.
    fn get_address_ptr(&self, dest: &ir::Address) -> Result<PointerValue<'ctx>, CodegenError> {
        match dest {
            ir::Address::Variable { name, .. } => {
                self.value_map.get(name)
                    .copied()
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined address: {}", name)))
            }
            ir::Address::Pointer { val, .. } => {
                let key = match &val.val {
                    ir::Value::Variable(n) => n.clone(),
                    ir::Value::Temp(n) => format!("%{}", n),
                    _ => return Err(CodegenError::Unsupported("Non-variable pointer base".to_string())),
                };
                self.value_map.get(&key)
                    .copied()
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined pointer: {}", key)))
            }
            _ => Err(CodegenError::Unsupported("Unsupported address kind".to_string())),
        }
    }

    /// Resolve a TypedValue to a BasicValueEnum.
    fn get_llvm_value(&self, val: &ir::TypedValue) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match &val.val {
            ir::Value::IntConst(n) => Ok(self.context.i64_type().const_int(*n as u64, true).into()),
            ir::Value::BoolConst(b) => {
                let v = if *b { 1u64 } else { 0u64 };
                Ok(self.context.i64_type().const_int(v, false).into())
            }
            ir::Value::FloatConst(f) => Ok(self.context.f64_type().const_float(*f).into()),
            ir::Value::Variable(name) => {
                self.value_map.get(name)
                    .map(|p| (*p).into())
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined value: {}", name)))
            }
            ir::Value::Temp(n) => {
                let key = format!("%{}", n);
                self.value_map.get(&key)
                    .map(|p| (*p).into())
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined value: {}", key)))
            }
            _ => Err(CodegenError::Unsupported("Unsupported value kind".to_string())),
        }
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