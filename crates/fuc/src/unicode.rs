use crate::types::*;
/// Provides UTF-8 processing utilities for the Fusion Standard Library.
pub struct UnicodeString {
    pub data: FString,
}

/// Represents a Unicode scalar value.
pub struct Scalar {
    pub value: FI64,
}

/// Validates if a byte slice is valid UTF-8.
pub fn is_valid_utf8(input: &FString) -> FBool {
    let bytes = input.as_bytes();
    let mut i: FSize = 0;
    let len = bytes.len();
    while i < len {
        let b = bytes[i] as i32;
        if b <= 0x7F {
            i += 1;
        } else if b & 0xE0 == 0xC0 {
            if i + 1 >= len || (bytes[i + 1] as i32 & 0xC0) != 0x80 { return false; }
            i += 2;
        } else if b & 0xF0 == 0xE0 {
            if i + 2 >= len || (bytes[i + 1] as i32 & 0xC0) != 0x80 || (bytes[i + 2] as i32 & 0xC0) != 0x80 { return false; }
            i += 3;
        } else if b & 0xF8 == 0xF0 {
            if i + 3 >= len || (bytes[i + 1] as i32 & 0xC0) != 0x80 || (bytes[i + 2] as i32 & 0xC0) != 0x80 || (bytes[i + 3] as i32 & 0xC0) != 0x80 { return false; }
            i += 4;
        } else {
            return false;
        }
    }
    return true;
}

/// Returns the number of Unicode code points (scalars) in the string.
pub fn count_scalars(input: &FString) -> FSize {
    let bytes = input.as_bytes();
    let mut count: FSize = 0;
    let mut i: FSize = 0;
    let len = bytes.len();
    while i < len {
        let b = bytes[i] as i32;
        if b & 0x80 == 0 { i += 1; }
        else if b & 0xE0 == 0xC0 { i += 2; }
        else if b & 0xF0 == 0xE0 { i += 3; }
        else if b & 0xF8 == 0xF0 { i += 4; }
        else { i += 1; }
        count += 1;
    }
    return count;
}