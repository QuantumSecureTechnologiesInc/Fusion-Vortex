// src/stdlib/io.rs - Standard I/O Module
#![allow(dead_code)]

use std::io::{self, Write};

/// Standard Input/Output Stream Abstraction
pub struct StdIO;

impl StdIO {
    /// Print string to stdout with newline
    pub fn println(s: &str) {
        println!("{}", s);
    }

    /// Print string to stdout without newline
    pub fn print(s: &str) {
        print!("{}", s);
        let _ = io::stdout().flush();
    }

    /// Read a line from stdin
    pub fn read_line() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();
        input.trim().to_string()
    }

    /// Explicit flush of stdout
    pub fn flush() {
        let _ = io::stdout().flush();
    }
}

/// Abstract Reader Trait
pub trait FusionRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn read_to_string(&mut self) -> std::io::Result<String>;
}

/// Abstract Writer Trait
pub trait FusionWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn flush(&mut self) -> std::io::Result<()>;
}
