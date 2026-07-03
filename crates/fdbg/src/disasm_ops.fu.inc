
/// x86-64 register names for 64-bit operand size.
fn reg_name_64(idx: u8) -> &'static str {
    match idx & 0x7 {
        0 => "rax", 1 => "rcx", 2 => "rdx", 3 => "rbx",
        4 => "rsp", 5 => "rbp", 6 => "rsi", 7 => "rdi",
        _ => "???",
    }
}

/// x86-64 register names for 32-bit operand size.
fn reg_name_32(idx: u8) -> &'static str {
    match idx & 0x7 {
        0 => "eax", 1 => "ecx", 2 => "edx", 3 => "ebx",
        4 => "esp", 5 => "ebp", 6 => "esi", 7 => "edi",
        _ => "???",
    }
}

/// x86-64 register names for 8-bit operand size.
fn reg_name_8(idx: u8) -> &'static str {
    match idx & 0x7 {
        0 => "al", 1 => "cl", 2 => "dl", 3 => "bl",
        4 => "ah", 5 => "ch", 6 => "dh", 7 => "bh",
        _ => "???",
    }
}

/// Decode operands for common x86-64 instructions.
/// Handles the most frequent encodings found in Fusion-compiled code.
pub fn decode_operands(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let mut pos: usize = 0;
    let mut rex_w = false;
    let mut rex_r = false;
    let mut rex_b = false;

    // Parse REX prefix
    if bytes[pos] >= 0x40 && bytes[pos] <= 0x4F {
        let rex = bytes[pos];
        rex_w = (rex & 0x08) != 0;
        rex_r = (rex & 0x04) != 0;
        rex_b = (rex & 0x01) != 0;
        pos += 1;
    }

    // Skip other prefixes
    while pos < bytes.len() {
        let b = bytes[pos];
        if b == 0x66 || b == 0x67 || b == 0xF0 || b == 0xF2 || b == 0xF3
            || b == 0x2E || b == 0x36 || b == 0x3E || b == 0x26 || b == 0x64 || b == 0x65
        {
            pos += 1;
        } else {
            break;
        }
    }

    if pos >= bytes.len() {
        return String::new();
    }

    let opcode = bytes[pos];
    pos += 1;

    // Choose register naming function based on REX.W
    let reg_fn = if rex_w { reg_name_64 } else { reg_name_32 };

    match opcode {
        // PUSH/POP register (0x50-0x5F): single register operand
        0x50..=0x57 => {
            let r = (opcode - 0x50) | if rex_b { 8 } else { 0 };
            format!("{}", reg_name_64(r))
        }
        0x58..=0x5F => {
            let r = (opcode - 0x58) | if rex_b { 8 } else { 0 };
            format!("{}", reg_name_64(r))
        }

        // MOV r/m, r (0x89) or MOV r, r/m (0x8B): two register operands via ModR/M
        0x89 | 0x8B => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = ((modrm >> 3) & 0x7) | if rex_r { 8 } else { 0 };
                let rm  = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                if mod_val == 0b11 {
                    // register-to-register
                    if opcode == 0x89 {
                        format!("{}, {}", reg_fn(rm), reg_fn(reg))
                    } else {
                        format!("{}, {}", reg_fn(reg), reg_fn(rm))
                    }
                } else if mod_val == 0b00 && (rm & 0x7) == 0b101 {
                    // RIP-relative
                    if pos + 5 <= bytes.len() {
                        let disp = i32::from_le_bytes([
                            bytes[pos + 1], bytes[pos + 2],
                            bytes[pos + 3], bytes[pos + 4],
                        ]);
                        if opcode == 0x89 {
                            format!("[rip + 0x{:x}], {}", disp as u32, reg_fn(reg))
                        } else {
                            format!("{}, [rip + 0x{:x}]", reg_fn(reg), disp as u32)
                        }
                    } else {
                        format!("{}, [rip+?]", reg_fn(if opcode == 0x8B { reg } else { rm }))
                    }
                } else if mod_val == 0b01 {
                    // [reg + disp8]
                    if pos + 1 < bytes.len() {
                        let disp = bytes[pos + 1] as i8;
                        if opcode == 0x89 {
                            format!("[{} + 0x{:x}], {}", reg_fn(rm), disp as u8, reg_fn(reg))
                        } else {
                            format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp as u8)
                        }
                    } else {
                        "?".to_string()
                    }
                } else if mod_val == 0b10 {
                    // [reg + disp32]
                    if pos + 5 <= bytes.len() {
                        let disp = i32::from_le_bytes([
                            bytes[pos + 1], bytes[pos + 2],
                            bytes[pos + 3], bytes[pos + 4],
                        ]);
                        if opcode == 0x89 {
                            format!("[{} + 0x{:x}], {}", reg_fn(rm), disp, reg_fn(reg))
                        } else {
                            format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp)
                        }
                    } else {
                        "?".to_string()
                    }
                } else {
                    // [reg] with no displacement
                    if opcode == 0x89 {
                        format!("[{}], {}", reg_fn(rm), reg_fn(reg))
                    } else {
                        format!("{}, [{}]", reg_fn(reg), reg_fn(rm))
                    }
                }
            } else {
                String::new()
            }
        }

        // LEA r, [mem] (0x8D)
        0x8D => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = ((modrm >> 3) & 0x7) | if rex_r { 8 } else { 0 };
                let rm  = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                if mod_val == 0b01 && pos + 1 < bytes.len() {
                    let disp = bytes[pos + 1] as i8;
                    format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp as u8)
                } else if mod_val == 0b10 && pos + 5 <= bytes.len() {
                    let disp = i32::from_le_bytes([
                        bytes[pos + 1], bytes[pos + 2],
                        bytes[pos + 3], bytes[pos + 4],
                    ]);
                    format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp)
                } else if mod_val == 0b00 && (rm & 0x7) == 0b101 && pos + 5 <= bytes.len() {
                    let disp = i32::from_le_bytes([
                        bytes[pos + 1], bytes[pos + 2],
                        bytes[pos + 3], bytes[pos + 4],
                    ]);
                    format!("{}, [rip + 0x{:x}]", reg_fn(reg), disp)
                } else {
                    format!("{}, [{}]", reg_fn(reg), reg_fn(rm))
                }
            } else {
                String::new()
            }
        }

        // SUB/ADD/AND/XOR/CMP r/m, imm8 (0x83)
        0x83 => {
            if pos + 1 < bytes.len() {
                let modrm = bytes[pos];
                let rm = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                let imm = bytes[pos + 1] as i8;
                format!("{}, 0x{:x}", reg_fn(rm), imm as u8)
            } else {
                "?".to_string()
            }
        }

        // MOV r, imm32/imm64 (0xB8-0xBF)
        0xB8..=0xBF => {
            let r = (opcode - 0xB8) | if rex_b { 8 } else { 0 };
            if rex_w && pos + 8 <= bytes.len() {
                let imm = u64::from_le_bytes([
                    bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3],
                    bytes[pos + 4], bytes[pos + 5], bytes[pos + 6], bytes[pos + 7],
                ]);
                format!("{}, 0x{:x}", reg_name_64(r), imm)
            } else if pos + 4 <= bytes.len() {
                let imm = u32::from_le_bytes([
                    bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3],
                ]);
                format!("{}, 0x{:x}", reg_fn(r), imm)
            } else {
                format!("{}", reg_fn(r))
            }
        }

        // MOV r8, imm8 (0xB0-0xB7)
        0xB0..=0xB7 => {
            let r = opcode - 0xB0;
            if pos < bytes.len() {
                format!("{}, 0x{:x}", reg_name_8(r), bytes[pos])
            } else {
                format!("{}", reg_name_8(r))
            }
        }

        // CALL/JMP near rel32 (0xE8, 0xE9)
        0xE8 | 0xE9 => {
            let name = if opcode == 0xE8 { "call" } else { "jmp" };
            if pos + 4 <= bytes.len() {
                let rel = i32::from_le_bytes([
                    bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3],
                ]);
                // Target address = current address + instr_len + rel
                // We approximate with just showing the relative offset
                let _ = name; // used in mnemonic, not operands
                format!("0x{:x}", rel)
            } else {
                "?".to_string()
            }
        }

        // JMP short rel8 (0xEB)
        0xEB => {
            if pos < bytes.len() {
                let rel = bytes[pos] as i8;
                format!("0x{:x}", rel as u8)
            } else {
                "?".to_string()
            }
        }

        // Conditional jumps rel8 (0x70-0x7F)
        0x70..=0x7F => {
            if pos < bytes.len() {
                let rel = bytes[pos] as i8;
                format!("0x{:x}", rel as u8)
            } else {
                "?".to_string()
            }
        }

        // PUSH imm8 (0x6A)
        0x6A => {
            if pos < bytes.len() {
                let imm = bytes[pos] as i8;
                format!("0x{:x}", imm as u8)
            } else {
                "?".to_string()
            }
        }

        // PUSH imm32 (0x68)
        0x68 => {
            if pos + 4 <= bytes.len() {
                let imm = u32::from_le_bytes([
                    bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3],
                ]);
                format!("0x{:x}", imm)
            } else {
                "?".to_string()
            }
        }

        // MOV r/m, imm32 (0xC7) or MOV r/m8, imm8 (0xC6)
        0xC7 => {
            if pos + 1 < bytes.len() {
                let modrm = bytes[pos];
                let rm = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                let imm_off = pos + 1;
                if imm_off + 4 <= bytes.len() {
                    let imm = u32::from_le_bytes([
                        bytes[imm_off], bytes[imm_off + 1],
                        bytes[imm_off + 2], bytes[imm_off + 3],
                    ]);
                    format!("{}, 0x{:x}", reg_fn(rm), imm)
                } else {
                    format!("{}", reg_fn(rm))
                }
            } else {
                "?".to_string()
            }
        }

        // INT3 (0xCC) — no operands
        0xCC => String::new(),
        // RET (0xC3) — no operands
        0xC3 => String::new(),

        // MOV r/m8, r8 (0x88)
        0x88 => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = (modrm >> 3) & 0x7;
                let rm  = modrm & 0x7;
                if mod_val == 0b11 {
                    format!("{}, {}", reg_name_8(rm), reg_name_8(reg))
                } else {
                    format!("[{}], {}", reg_fn(rm), reg_name_8(reg))
                }
            } else {
                String::new()
            }
        }

        // MOV r, r/m8 (0x8A) or MOV r, r/m (0x8B already handled)
        0x8A => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = (modrm >> 3) & 0x7;
                let rm  = modrm & 0x7;
                if mod_val == 0b11 {
                    format!("{}, {}", reg_name_8(reg), reg_name_8(rm))
                } else {
                    format!("{}, [{}]", reg_name_8(reg), reg_fn(rm))
                }
            } else {
                String::new()
            }
        }

        // TEST r/m, r (0x85)
        0x85 => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = ((modrm >> 3) & 0x7) | if rex_r { 8 } else { 0 };
                let rm  = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                if mod_val == 0b11 {
                    format!("{}, {}", reg_fn(rm), reg_fn(reg))
                } else {
                    format!("[{}], {}", reg_fn(rm), reg_fn(reg))
                }
            } else {
                String::new()
            }
        }

        // ADD/SUB/CMP/AND/OR/XOR r, imm32 (0x81)
        0x81 => {
            if pos + 1 < bytes.len() {
                let modrm = bytes[pos];
                let rm = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                let imm_off = pos + 1;
                if imm_off + 4 <= bytes.len() {
                    let imm = u32::from_le_bytes([
                        bytes[imm_off], bytes[imm_off + 1],
                        bytes[imm_off + 2], bytes[imm_off + 3],
                    ]);
                    format!("{}, 0x{:x}", reg_fn(rm), imm)
                } else {
                    format!("{}", reg_fn(rm))
                }
            } else {
                "?".to_string()
            }
        }

        // MOVSXD (0x63)
        0x63 => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = ((modrm >> 3) & 0x7) | if rex_r { 8 } else { 0 };
                let rm  = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                if mod_val == 0b11 {
                    format!("{}, {}", reg_name_64(reg), reg_name_32(rm))
                } else {
                    format!("{}, [{}]", reg_name_64(reg), reg_fn(rm))
                }
            } else {
                String::new()
            }
        }

        // Default: no operand decode
        _ => String::new(),
    }
}

/// x86-64 register names for 64-bit operand size.
fn reg_name_64(idx: u8) -> &'static str {
    match idx & 0x7 {
        0 => "rax", 1 => "rcx", 2 => "rdx", 3 => "rbx",
        4 => "rsp", 5 => "rbp", 6 => "rsi", 7 => "rdi",
        _ => "???",
    }
}

/// x86-64 register names for 32-bit operand size.
fn reg_name_32(idx: u8) -> &'static str {
    match idx & 0x7 {
        0 => "eax", 1 => "ecx", 2 => "edx", 3 => "ebx",
        4 => "esp", 5 => "ebp", 6 => "esi", 7 => "edi",
        _ => "???",
    }
}

/// x86-64 register names for 8-bit operand size.
fn reg_name_8(idx: u8) -> &'static str {
    match idx & 0x7 {
        0 => "al", 1 => "cl", 2 => "dl", 3 => "bl",
        4 => "ah", 5 => "ch", 6 => "dh", 7 => "bh",
        _ => "???",
    }
}

/// Decode operands for common x86-64 instructions.
pub fn decode_operands(bytes: &[u8]) -> String {
    if bytes.is_empty() { return String::new(); }
    let mut pos: usize = 0;
    let mut rex_w = false;
    let mut rex_r = false;
    let mut rex_b = false;
    // Parse REX prefix
    if bytes[pos] >= 0x40 && bytes[pos] <= 0x4F {
        let rex = bytes[pos];
        rex_w = (rex & 0x08) != 0;
        rex_r = (rex & 0x04) != 0;
        rex_b = (rex & 0x01) != 0;
        pos += 1;
    }
    // Skip other prefixes
    while pos < bytes.len() {
        let b = bytes[pos];
        if b == 0x66 || b == 0x67 || b == 0xF0 || b == 0xF2 || b == 0xF3
            || b == 0x2E || b == 0x36 || b == 0x3E || b == 0x26 || b == 0x64 || b == 0x65 {
            pos += 1;
        } else { break; }
    }
    if pos >= bytes.len() { return String::new(); }
    let opcode = bytes[pos];
    pos += 1;
    let reg_fn = if rex_w { reg_name_64 } else { reg_name_32 };
    match opcode {
        0x50..=0x57 => format!("{}", reg_name_64((opcode - 0x50) | if rex_b { 8 } else { 0 })),
        0x58..=0x5F => format!("{}", reg_name_64((opcode - 0x58) | if rex_b { 8 } else { 0 })),
        0x89 | 0x8B => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = ((modrm >> 3) & 0x7) | if rex_r { 8 } else { 0 };
                let rm  = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                if mod_val == 0b11 {
                    if opcode == 0x89 { format!("{}, {}", reg_fn(rm), reg_fn(reg)) }
                    else { format!("{}, {}", reg_fn(reg), reg_fn(rm)) }
                } else if mod_val == 0b01 && pos + 1 < bytes.len() {
                    let disp = bytes[pos + 1] as i8;
                    if opcode == 0x89 { format!("[{} + 0x{:x}], {}", reg_fn(rm), disp as u8, reg_fn(reg)) }
                    else { format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp as u8) }
                } else {
                    if opcode == 0x89 { format!("[{}], {}", reg_fn(rm), reg_fn(reg)) }
                    else { format!("{}, [{}]", reg_fn(reg), reg_fn(rm)) }
                }
            } else { String::new() }
        }
        0x8D => {
            if pos < bytes.len() {
                let modrm = bytes[pos];
                let mod_val = (modrm >> 6) & 0x3;
                let reg = ((modrm >> 3) & 0x7) | if rex_r { 8 } else { 0 };
                let rm  = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                if mod_val == 0b01 && pos + 1 < bytes.len() {
                    let disp = bytes[pos + 1] as i8;
                    format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp as u8)
                } else if mod_val == 0b10 && pos + 5 <= bytes.len() {
                    let disp = i32::from_le_bytes([bytes[pos+1], bytes[pos+2], bytes[pos+3], bytes[pos+4]]);
                    format!("{}, [{} + 0x{:x}]", reg_fn(reg), reg_fn(rm), disp)
                } else { format!("{}, [{}]", reg_fn(reg), reg_fn(rm)) }
            } else { String::new() }
        }
        0x83 => {
            if pos + 1 < bytes.len() {
                let modrm = bytes[pos];
                let rm = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                let imm = bytes[pos + 1] as i8;
                format!("{}, 0x{:x}", reg_fn(rm), imm as u8)
            } else { "?".to_string() }
        }
        0xB8..=0xBF => {
            let r = (opcode - 0xB8) | if rex_b { 8 } else { 0 };
            if rex_w && pos + 8 <= bytes.len() {
                let imm = u64::from_le_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3], bytes[pos+4], bytes[pos+5], bytes[pos+6], bytes[pos+7]]);
                format!("{}, 0x{:x}", reg_name_64(r), imm)
            } else if pos + 4 <= bytes.len() {
                let imm = u32::from_le_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]);
                format!("{}, 0x{:x}", reg_fn(r), imm)
            } else { format!("{}", reg_fn(r)) }
        }
        0xB0..=0xB7 => {
            let r = opcode - 0xB0;
            if pos < bytes.len() { format!("{}, 0x{:x}", reg_name_8(r), bytes[pos]) }
            else { format!("{}", reg_name_8(r)) }
        }
        0xE8 | 0xE9 => {
            if pos + 4 <= bytes.len() {
                let rel = i32::from_le_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]);
                format!("0x{:x}", rel)
            } else { "?".to_string() }
        }
        0xEB | 0x70..=0x7F => {
            if pos < bytes.len() { format!("0x{:x}", bytes[pos] as i8 as u8) }
            else { "?".to_string() }
        }
        0x6A => {
            if pos < bytes.len() { format!("0x{:x}", bytes[pos] as i8 as u8) }
            else { "?".to_string() }
        }
        0x68 => {
            if pos + 4 <= bytes.len() {
                let imm = u32::from_le_bytes([bytes[pos], bytes[pos+1], bytes[pos+2], bytes[pos+3]]);
                format!("0x{:x}", imm)
            } else { "?".to_string() }
        }
        0xCC | 0xC3 => String::new(),
        0xC7 => {
            if pos + 1 < bytes.len() {
                let modrm = bytes[pos];
                let rm = (modrm & 0x7) | if rex_b { 8 } else { 0 };
                let imm_off = pos + 1;
                if imm_off + 4 <= bytes.len() {
                    let imm = u32::from_le_bytes([bytes[imm_off], bytes[imm_off+1], bytes[imm_off+2], bytes[imm_off+3]]);
                    format!("{}, 0x{:x}", reg_fn(rm), imm)
                } else { format!("{}", reg_fn(rm)) }
            } else { "?".to_string() }
        }
        _ => String::new(),
    }
}