// src/stdlib/fs.rs - Filesystem Module
#![allow(dead_code)]

use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Secure Filesystem Operations
pub struct FileSystem;

impl FileSystem {
    /// Read entire file to string
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
        // In a real secure FS, we would check permissions/sandbox here
        fs::read_to_string(path)
    }

    /// Write string to file (overwriting)
    pub fn write_str<P: AsRef<Path>>(path: P, contents: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(contents.as_bytes())
    }

    /// Append string to file
    pub fn append_str<P: AsRef<Path>>(path: P, contents: &str) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        file.write_all(contents.as_bytes())
    }

    /// Check if file exists
    pub fn exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }

    /// Create directory
    pub fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    /// Remove file
    pub fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::remove_file(path)
    }
}
