//! Fusion C FFI Bridge
//!
//! Provides utilities for C interoperability.

#![allow(dead_code)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Convert a C string to a Rust String
pub fn c_str_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr).to_str().ok().map(String::from) }
}

/// Convert a Rust string to a C string (caller must free)
pub fn string_to_c_str(s: &str) -> *mut c_char {
    CString::new(s)
        .map(|c| c.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Free a C string allocated by this module
pub unsafe fn free_c_str(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}
