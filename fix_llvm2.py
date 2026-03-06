import re

with open('crates/fuc/src/codegen/llvm.fu', 'r') as f:
    content = f.read()

# Replace all occurrences of .try_as_basic_value().left().unwrap().into_int_value()
# The new API returns ValueKind which we need to match on

# Pattern 1: For strcmp results
old1 = '''let cmp_val = if let Ok(inkwell::values::BasicValueEnum::IntValue(v)) = call_site.try_as_basic_value() {
                                        v
                                    } else {
                                        panic!("Expected int return value")
                                    };'''

new1 = '''let cmp_val = match call_site.try_as_basic_value() {
                                        inkwell::values::ValueKind::IntValue(v) => v,
                                        _ => panic!("strcmp should return int"),
                                    };'''

content = content.replace(old1, new1)

# Pattern 2: For the if let with .left() that we partially replaced
old2 = '''if let Ok(basic_val) = call_site.try_as_basic_value()'''
new2 = '''if let inkwell::values::ValueKind::BasicValue(basic_val) = call_site.try_as_basic_value()'''

content = content.replace(old2, new2)

with open('crates/fuc/src/codegen/llvm.fu', 'w') as f:
    f.write(content)

print('Fixed LLVM codegen v2')
