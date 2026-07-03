// LLVM Backend - requires inkwell crate (feature = "llvm")
#![cfg(feature = "llvm")]

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue, PointerValue, FunctionValue, BasicValue, StructValue};
use inkwell::types::{BasicType, BasicTypeEnum, FunctionType, StructType};
use inkwell::IntPredicate;
use inkwell::AddressSpace;
use crate::ir::{self, *};
use crate::codegen::{Backend, CodegenConfig, CodegenError};
use std::collections::HashMap;

pub struct LlvmBackend<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    config: CodegenConfig,
    value_map: HashMap<String, BasicValueEnum<'ctx>>,
    struct_types: HashMap<String, StructType<'ctx>>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> LlvmBackend<'ctx> {
    pub fn new(context: &'ctx Context, name: &str, config: &CodegenConfig) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        Self {
            context,
            module,
            builder,
            config: config.clone(),
            value_map: HashMap::new(),
            struct_types: HashMap::new(),
            current_function: None,
        }
    }

    /// Compile an entire IR module.
    pub fn compile_module(&mut self, ir_module: &IrModule) -> Result<(), CodegenError> {
        // Declare struct types first
        for sd in &ir_module.structs {
            self.declare_struct(sd)?;
        }

        // Declare extern functions
        for ext in &ir_module.externs {
            self.declare_extern(ext)?;
        }

        // Compile functions
        for func in &ir_module.functions {
            self.compile_function(func)?;
        }

        Ok(())
    }

    /// Declare a struct type in LLVM.
    fn declare_struct(&mut self, sd: &IrStructDef) -> Result<(), CodegenError> {
        let field_types: Vec<BasicTypeEnum<'ctx>> = sd.fields.iter()
            .map(|(_, ty)| self.ir_type_to_llvm(ty))
            .collect::<Result<Vec<_>, _>>()?;
        let struct_type = self.context.struct_type(&field_types, false);
        self.struct_types.insert(sd.name.clone(), struct_type);
        Ok(())
    }

    /// Declare an extern function.
    fn declare_extern(&mut self, ext: &IrExtern) -> Result<(), CodegenError> {
        let ret_type = self.ir_type_to_llvm(&ext.return_type)?;
        let param_types: Vec<BasicTypeEnum<'ctx>> = ext.params.iter()
            .map(|t| self.ir_type_to_llvm(t))
            .collect::<Result<Vec<_>, _>>()?;
        let fn_type = ret_type.fn_type(&param_types, false);
        self.module.add_function(&ext.name, fn_type, None);
        Ok(())
    }

    /// Compile a single IR function.
    pub fn compile_function(&mut self, func: &IrFunction) -> Result<FunctionValue<'ctx>, CodegenError> {
        self.value_map.clear();

        let ret_type = self.context.void_type().into();
        let fn_type = ret_type.fn_type(&[], false);
        let llvm_func = self.module.add_function(&func.name, fn_type, None);
        self.current_function = Some(llvm_func);

        // Create LLVM basic blocks
        let mut llvm_blocks: Vec<inkwell::basic_block::BasicBlock<'ctx>> = Vec::new();
        for block in &func.blocks {
            let bb = self.context.append_basic_block(llvm_func, &block.label);
            llvm_blocks.push(bb);
        }

        // Compile each block
        for (i, block) in func.blocks.iter().enumerate() {
            self.builder.position_at_end(llvm_blocks[i]);

            for instr in &block.instrs {
                self.compile_instruction(instr)?;
            }

            self.compile_terminator(&block.terminator, &llvm_blocks)?;
        }

        Ok(llvm_func)
    }

    /// Compile a single IR instruction.
    fn compile_instruction(&mut self, instr: &Instruction) -> Result<(), CodegenError> {
        match instr {
            Instruction::BinaryOperation { dest, op, op1, op2 } => {
                let left = self.resolve_value(op1)?;
                let right = self.resolve_value(op2)?;
                let result = match (left, right) {
                    (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                        let val = match op {
                            BinaryOp::Add => self.builder.build_int_add(l, r, "add"),
                            BinaryOp::Sub => self.builder.build_int_sub(l, r, "sub"),
                            BinaryOp::Mul => self.builder.build_int_mul(l, r, "mul"),
                            BinaryOp::Div => self.builder.build_int_signed_div(l, r, "div"),
                            BinaryOp::Mod => self.builder.build_int_signed_rem(l, r, "mod"),
                            BinaryOp::Eq => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                            BinaryOp::Neq => self.builder.build_int_compare(IntPredicate::NE, l, r, "neq"),
                            BinaryOp::Lt => self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt"),
                            BinaryOp::Gt => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt"),
                            BinaryOp::Le => self.builder.build_int_compare(IntPredicate::SLE, l, r, "le"),
                            BinaryOp::Ge => self.builder.build_int_compare(IntPredicate::SGE, l, r, "ge"),
                            BinaryOp::And => {
                                let l_bool = self.builder.build_int_compare(IntPredicate::NE, l, self.context.i64_type().const_int(0, false), "l_bool");
                                let r_bool = self.builder.build_int_compare(IntPredicate::NE, r, self.context.i64_type().const_int(0, false), "r_bool");
                                let and_val = self.builder.build_and(l_bool, r_bool, "and");
                                self.builder.build_int_z_extend(and_val, self.context.i64_type(), "and_ext")
                            }
                            BinaryOp::Or => {
                                let l_bool = self.builder.build_int_compare(IntPredicate::NE, l, self.context.i64_type().const_int(0, false), "l_bool");
                                let r_bool = self.builder.build_int_compare(IntPredicate::NE, r, self.context.i64_type().const_int(0, false), "r_bool");
                                let or_val = self.builder.build_or(l_bool, r_bool, "or");
                                self.builder.build_int_z_extend(or_val, self.context.i64_type(), "or_ext")
                            }
                        };
                        val.map(BasicValueEnum::from).map_err(|e| CodegenError::LlvmError(e.to_string()))?
                    }
                    _ => return Err(CodegenError::Unsupported("Binary op on non-int types".to_string())),
                };
                self.store_value(dest, result);
            }
            Instruction::Call { dest, func_name, args } => {
                let llvm_func = self.module.get_function(func_name)
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined function: {}", func_name)))?;
                let llvm_args: Vec<BasicValueEnum<'ctx>> = args.iter()
                    .map(|a| self.resolve_value(a))
                    .collect::<Result<Vec<_>, _>>()?;
                let call_result = self.builder.build_call(llvm_func, &llvm_args, "call")
                    .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                if let Some(d) = dest {
                    if let Some(val) = call_result.try_as_basic_value().left() {
                        self.store_value(d, val);
                    }
                }
            }
            Instruction::Load { dest, src } => {
                let ptr = self.resolve_address(src)?;
                let loaded = self.builder.build_load(ptr, &dest.ty.to_string())
                    .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                self.store_value(dest, loaded);
            }
            Instruction::Store { dest, val } => {
                let ptr = self.resolve_address(dest)?;
                let llvm_val = self.resolve_value(val)?;
                self.builder.build_store(ptr, llvm_val)
                    .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
            }
            Instruction::GetElementPtr { dest, base_ptr, index, element_ty } => {
                let ptr_val = self.resolve_value(base_ptr)?;
                let ptr = ptr_val.into_pointer_value();
                let idx = self.resolve_value(index)?;
                let idx_val = idx.into_int_value();
                let elem_llvm_ty = self.ir_type_to_llvm(element_ty)?;
                let gep = unsafe {
                    self.builder.build_gep(ptr, &[idx_val], "gep")
                }.map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                self.store_value(dest, gep.into());
            }
            Instruction::GetFieldPtr { dest, base_ptr, field_index, field_ty, struct_name } => {
                let ptr_val = self.resolve_value(base_ptr)?;
                let ptr = ptr_val.into_pointer_value();
                let zero = self.context.i32_type().const_int(0, false);
                let idx = self.context.i32_type().const_int(*field_index as u64, false);
                let gep = unsafe {
                    self.builder.build_gep(ptr, &[zero, idx], "field_ptr")
                }.map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                self.store_value(dest, gep.into());
            }
            Instruction::Alloca { dest, ty } => {
                let llvm_ty = self.ir_type_to_llvm(ty)?;
                let alloca = self.builder.build_alloca(llvm_ty, "alloca")
                    .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                self.store_value(dest, alloca.into());
            }
            Instruction::Copy { dest, src } => {
                let val = self.resolve_value(src)?;
                self.store_value(dest, val);
            }
            Instruction::UnaryNot { dest, operand } => {
                let val = self.resolve_value(operand)?;
                let int_val = val.into_int_value();
                let zero = self.context.i64_type().const_int(0, false);
                let cmp = self.builder.build_int_compare(IntPredicate::EQ, int_val, zero, "not");
                let ext = self.builder.build_int_z_extend(cmp, self.context.i64_type(), "not_ext");
                self.store_value(dest, ext.into());
            }
            Instruction::Phi { dest, incoming } => {
                // Build phi node
                let llvm_ty = self.ir_type_to_llvm(&dest.ty)?;
                let phi = self.builder.build_phi(llvm_ty, "phi");
                // Incoming values are added in compile_terminator/block handling
                self.store_value(dest, phi.as_basic_value());
            }
            Instruction::MakeClosure { dest, func_name, captured } => {
                // Simplified: store function pointer as first capture
                let llvm_func = self.module.get_function(func_name)
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined function: {}", func_name)))?;
                let ptr = llvm_func.as_global_value().as_pointer_value();
                self.store_value(dest, ptr.into());
            }
            Instruction::Comment(_) => {
                // No-op
            }
            Instruction::GetAddress { dest, var_name, is_mutable: _ } => {
                if let Some(val) = self.value_map.get(var_name) {
                    if let BasicValueEnum::PointerValue(ptr) = val {
                        self.value_map.insert(dest.clone(), (*ptr).into());
                    }
                }
            }
        }
        Ok(())
    }

    /// Compile a terminator instruction.
    fn compile_terminator(
        &mut self,
        term: &Terminator,
        llvm_blocks: &[inkwell::basic_block::BasicBlock<'ctx>],
    ) -> Result<(), CodegenError> {
        match term {
            Terminator::Jump(target) => {
                self.builder.build_unconditional_branch(llvm_blocks[*target])
                    .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
            }
            Terminator::ConditionalJump { cond, then_block, else_block } => {
                let cond_val = self.resolve_value(cond)?;
                let cond_int = cond_val.into_int_value();
                let is_true = self.builder.build_int_compare(
                    IntPredicate::NE,
                    cond_int,
                    self.context.i64_type().const_int(0, false),
                    "cond",
                );
                self.builder.build_conditional_branch(
                    is_true,
                    llvm_blocks[*then_block],
                    llvm_blocks[*else_block],
                ).map_err(|e| CodegenError::LlvmError(e.to_string()))?;
            }
            Terminator::Return(val) => {
                if let Some(v) = val {
                    let llvm_val = self.resolve_value(v)?;
                    self.builder.build_return(Some(&llvm_val))
                        .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                } else {
                    self.builder.build_return(None)
                        .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
                }
            }
            Terminator::Unreachable => {
                self.builder.build_unreachable()
                    .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
            }
        }
        Ok(())
    }

    /// Resolve a TypedValue to an LLVM BasicValueEnum.
    fn resolve_value(&self, tv: &TypedValue) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match &tv.val {
            Value::IntConst(n) => {
                let ty = self.ir_type_to_llvm(&tv.ty)?;
                Ok(ty.into_int_type().const_int(*n as u64, true).into())
            }
            Value::BoolConst(b) => {
                let val = if *b { 1u64 } else { 0u64 };
                Ok(self.context.i64_type().const_int(val, false).into())
            }
            Value::StringConst(s) => {
                let global = self.module.add_global(
                    self.context.i8_type().array_type(s.len() as u32),
                    None,
                    &s,
                );
                let ptr = global.as_pointer_value();
                Ok(ptr.into())
            }
            Value::FloatConst(f) => {
                Ok(self.context.f64_type().const_float(*f).into())
            }
            Value::Variable(name) | Value::Temp(name) => {
                let key = match &tv.val {
                    Value::Variable(n) => n.clone(),
                    Value::Temp(n) => format!("%{}", n),
                    _ => unreachable!(),
                };
                self.value_map.get(&key)
                    .cloned()
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined value: {}", key)))
            }
        }
    }

    /// Resolve an Address to an LLVM PointerValue.
    fn resolve_address(&self, addr: &Address) -> Result<PointerValue<'ctx>, CodegenError> {
        match addr {
            Address::Variable { name, .. } => {
                self.value_map.get(name)
                    .and_then(|v| if let BasicValueEnum::PointerValue(p) = v { Some(*p) } else { None })
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined address: {}", name)))
            }
            Address::Pointer { val, .. } => {
                let ptr_val = self.resolve_value(val)?;
                Ok(ptr_val.into_pointer_value())
            }
            _ => Err(CodegenError::Unsupported("Complex address resolution".to_string())),
        }
    }

    /// Store a value in the value map.
    fn store_value(&mut self, tv: &TypedValue, val: BasicValueEnum<'ctx>) {
        let key = match &tv.val {
            Value::Temp(n) => format!("%{}", n),
            Value::Variable(n) => n.clone(),
            _ => return,
        };
        self.value_map.insert(key, val);
    }

    /// Convert an IR type to an LLVM type.
    fn ir_type_to_llvm(&self, ty: &Type) -> Result<BasicTypeEnum<'ctx>, CodegenError> {
        match ty {
            Type::Int => Ok(self.context.i64_type().into()),
            Type::Bool => Ok(self.context.i64_type().into()),
            Type::Float => Ok(self.context.f64_type().into()),
            Type::Void => Ok(self.context.void_type().into()),
            Type::String | Type::Slice(_) => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            Type::Pointer(inner) => {
                let inner_ty = self.ir_type_to_llvm(inner)?;
                Ok(inner_ty.ptr_type(AddressSpace::default()).into())
            }
            Type::Array(elem, len) => {
                let elem_ty = self.ir_type_to_llvm(elem)?;
                Ok(elem_ty.array_type(*len as u32).into())
            }
            Type::Struct(name) => {
                self.struct_types.get(name)
                    .cloned()
                    .map(|t| t.into())
                    .ok_or_else(|| CodegenError::LlvmError(format!("Undefined struct: {}", name)))
            }
            _ => Ok(self.context.i64_type().into()), // Default fallback
        }
    }

    /// Write the module to a file.
    pub fn write_to_file(&self, path: &str) -> Result<(), CodegenError> {
        self.module.print_to_file(path)
            .map_err(|e| CodegenError::LlvmError(e.to_string()))
    }

    /// Write the module to memory as LLVM IR text.
    pub fn write_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    /// Write the module to an object file.
    pub fn write_object_file(&self, path: &str) -> Result<(), CodegenError> {
        // Use LLVM's TargetMachine for native object emission
        use inkwell::targets::{Target, TargetMachine, FileType, RelocMode, CodeModel};
        use inkwell::OptimizationLevel;

        Target::initialize_native(&inkwell::targets::InitializationConfig::default())
            .map_err(|e| CodegenError::LlvmError(e.to_string()))?;

        let triple = inkwell::targets::TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|e| CodegenError::LlvmError(e.to_string()))?;
        let target_machine = target.create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::from(self.config.optimization_level),
            RelocMode::Default,
            CodeModel::Default,
        ).ok_or_else(|| CodegenError::LlvmError("Failed to create target machine".to_string()))?;

        target_machine.write_to_file(&self.module, FileType::Object, std::path::Path::new(path))
            .map_err(|e| CodegenError::LlvmError(e.to_string()))
    }
}

impl<'ctx> Backend for LlvmBackend<'ctx> {
    fn compile_module(&mut self, module: &IrModule) -> Result<Vec<u8>, CodegenError> {
        // Compile to memory
        self.compile_module(module)?;
        // Return empty bytes for now - object file emission is separate
        Ok(Vec::new())
    }
}