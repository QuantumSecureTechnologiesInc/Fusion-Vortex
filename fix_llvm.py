import re

with open('crates/fuc/src/codegen/llvm.fu', 'r') as f:
    content = f.read()

# Replace all occurrences of .try_as_basic_value().left().unwrap().into_int_value()
# with a pattern match on the ValueKind

# Pattern 1: For strcmp results with .left().unwrap().into_int_value()
pattern1 = r'''let cmp_val = call_site
                                        \.try_as_basic_value\(\)
                                        \.left\(\)
                                        \.unwrap\(\)
                                        \.into_int_value\(\);'''

replacement1 = '''let cmp_val = if let Ok(inkwell::values::BasicValueEnum::IntValue(v)) = call_site.try_as_basic_value() {
                                        v
                                    } else {
                                        panic!("Expected int return value")
                                    };'''

content = re.sub(pattern1, replacement1, content)

# Pattern 2: For the if let with .left()
pattern2 = r'''if let Some\(basic_val\) = call_site
                                \.try_as_basic_value\(\)
                                \.left\(\)'''

replacement2 = '''if let Ok(basic_val) = call_site.try_as_basic_value()'''

content = re.sub(pattern2, replacement2, content)

with open('crates/fuc/src/codegen/llvm.fu', 'w') as f:
    f.write(content)

print('Fixed LLVM codegen')
