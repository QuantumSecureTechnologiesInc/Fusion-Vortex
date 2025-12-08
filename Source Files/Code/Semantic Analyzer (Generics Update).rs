// src/semantic_analyzer/mod.rs - Type Checking, Scope, and Borrow Checking (Updated for Generics)

// ... (existing use statements and SemanticAnalyzer struct definition) ...

// New structure to store all available traits and their required methods
struct TraitDefinition {
    name: String,
    method_signatures: HashMap<String, MethodSignature>,
}

pub struct SemanticAnalyzer {
    // ... (existing fields: symbol_table, borrow_checker, type_errors, borrow_errors, etc.) ...

    // New: Repository of all defined traits
    trait_definitions: HashMap<String, TraitDefinition>,
}

// ... (existing SymbolTable, BorrowChecker, and SemanticAnalyzer::new methods) ...

impl SemanticAnalyzer {
    // ... (existing analyze method) ...

    fn check_declaration(&mut self, decl: &Declaration) -> Result<(), Vec<String>> {
        match decl {
            Declaration::Trait { name, methods } => {
                // Register the new trait definition for later use in bounds checking
                let mut method_map = HashMap::new();
                for method in methods {
                    method_map.insert(method.name.clone(), method.clone());
                }
                self.trait_definitions.insert(
                    name.clone(),
                    TraitDefinition {
                        name: name.clone(),
                        method_signatures: method_map,
                    },
                );
                Ok(())
            }
            Declaration::Function {
                name,
                generic_params,
                where_bounds,
                params,
                return_type,
                body,
                ..
            } => {
                self.symbol_table.push_scope();

                // 1. Resolve Generic Constraints
                let constraints = self.resolve_generic_constraints(generic_params, where_bounds)?;

                // 2. Register parameters (which might use TypeParameter types)
                for param in params {
                    self.symbol_table
                        .define(param.name.clone(), param.param_type.clone(), false);
                }

                // 3. Check function body (now expressions involving generic types must respect constraints)
                self.check_block(body)?;

                self.symbol_table.pop_scope();
                // ... (reset attributes)
                Ok(())
            }
            // ... (Class and other declarations remain the same)
            _ => Ok(()),
        }
    }

    /// Resolves generic constraints and verifies that the required traits exist.
    fn resolve_generic_constraints(
        &self,
        generic_params: &[String],
        where_bounds: &[TraitBound],
    ) -> Result<HashMap<String, TraitBound>, Vec<String>> {
        let mut constraints = HashMap::new();
        let mut errors = Vec::new();

        for bound in where_bounds {
            // Check that the constraint trait actually exists
            if !self.trait_definitions.contains_key(&bound.trait_name) {
                errors.push(format!(
                    "Trait '{}' required by generic type '{}' is not defined.",
                    bound.trait_name, bound.type_name
                ));
            }
            // Check that the type being constrained is actually a generic parameter of the function
            if !generic_params.contains(&bound.type_name) {
                errors.push(format!(
                    "Trait bound placed on non-generic type parameter: '{}'.",
                    bound.type_name
                ));
            }

            constraints.insert(bound.type_name.clone(), bound.clone());
        }

        if errors.is_empty() {
            Ok(constraints)
        } else {
            Err(errors)
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<Type, Vec<String>> {
        match expr {
            // ... (Literal, Variable, BinaryOp, etc.) ...

            // 4. Check Method Calls on Generic Types
            Expression::MethodCall {
                object,
                method,
                args,
            } => {
                let object_type = self.check_expression(object)?;

                if let Type::TypeParameter(param_name) = object_type {
                    // Check if the generic type 'param_name' has a constraint that requires 'method'

                    // Simplified lookup logic:
                    if let Some(bound) = self.get_constraint(&param_name) {
                        if let Some(trait_def) = self.trait_definitions.get(&bound.trait_name) {
                            if trait_def.method_signatures.contains_key(method) {
                                // Success: The trait bound guarantees the existence of this method.
                                // We would now check the arguments and return type against the signature.
                                return Ok(Type::Unknown); // Placeholder for resolved return type
                            }
                        }
                    }
                    // Failure: Method not guaranteed by bounds
                    self.type_errors.push(format!("Method '{}' is not available on generic type '{}' because it is not required by any trait bound.", method, param_name));
                    return Ok(Type::Unknown);
                }
                // ... (Continue checking for concrete types)

                Ok(Type::Unknown)
            }
            _ => self.default_expression_check(expr),
        }
    }

    // --- Utility Methods ---

    fn get_constraint(&self, type_name: &str) -> Option<&TraitBound> {
        // Placeholder for looking up constraints in the current function scope
        None
    }

    fn default_expression_check(&self, expr: &Expression) -> Result<Type, Vec<String>> {
        // ... (existing default checks)
        Ok(Type::Unknown)
    }

    // ... (literal_type, types_compatible methods) ...
}
