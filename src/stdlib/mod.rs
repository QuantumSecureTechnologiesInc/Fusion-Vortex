// src/stdlib/mod.rs - Standard Library Kernel

pub const CORE_LIBS: &str = r#"
extern fn malloc(size: int) -> int;
extern fn free(ptr: int) -> void;
extern fn realloc(ptr: int, size: int) -> int;
extern fn memcpy(dest: int, src: int, n: int) -> void;
extern fn strlen(s: string) -> int;
"#;

pub mod fs;
pub mod io;

pub mod types {
    // Basic types are built-in, but we can define wrappers here
}
