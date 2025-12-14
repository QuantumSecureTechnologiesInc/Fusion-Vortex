use bytes::{Bytes, BytesMut};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RespError {
    #[error("Incomplete")]
    Incomplete,
    #[error("Invalid protocol")]
    InvalidProtocol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<Bytes>), // None represents Null Bulk String
    Array(Option<Vec<Value>>), // None represents Null Array
}

impl Value {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Value::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            Value::Error(msg) => format!("-{}\r\n", msg).into_bytes(),
            Value::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            Value::BulkString(data) => match data {
                Some(b) => {
                    let mut vec = format!("${}\r\n", b.len()).into_bytes();
                    vec.extend_from_slice(b);
                    vec.extend_from_slice(b"\r\n");
                    vec
                }
                None => b"$-1\r\n".to_vec(),
            },
            Value::Array(vals) => match vals {
                Some(v) => {
                    let mut vec = format!("*{}\r\n", v.len()).into_bytes();
                    for val in v {
                        vec.extend(val.serialize());
                    }
                    vec
                }
                None => b"*-1\r\n".to_vec(),
            },
        }
    }
}

pub struct RespParser;

impl RespParser {
    pub fn parse(buffer: &mut BytesMut) -> Result<Option<(Value, usize)>, RespError> {
        if buffer.is_empty() {
            return Ok(None);
        }

        let (value, len) = match buffer[0] {
            b'+' => Self::parse_simple_string(buffer)?,
            b'-' => Self::parse_error(buffer)?,
            b':' => Self::parse_integer(buffer)?,
            b'$' => Self::parse_bulk_string(buffer)?,
            b'*' => Self::parse_array(buffer)?,
            _ => return Err(RespError::InvalidProtocol),
        };

        Ok(Some((value, len)))
    }

    fn read_line(buffer: &[u8]) -> Result<Option<(&[u8], usize)>, RespError> {
        for i in 0..buffer.len().saturating_sub(1) {
            if buffer[i] == b'\r' && buffer[i + 1] == b'\n' {
                return Ok(Some((&buffer[0..i], i + 2)));
            }
        }
        Ok(None)
    }

    fn parse_simple_string(buffer: &BytesMut) -> Result<(Value, usize), RespError> {
        if let Some((line, len)) = Self::read_line(&buffer[1..])? {
            let s = String::from_utf8_lossy(line).to_string();
            Ok((Value::SimpleString(s), len + 1))
        } else {
            Err(RespError::Incomplete)
        }
    }

    fn parse_error(buffer: &BytesMut) -> Result<(Value, usize), RespError> {
        if let Some((line, len)) = Self::read_line(&buffer[1..])? {
            let s = String::from_utf8_lossy(line).to_string();
            Ok((Value::Error(s), len + 1))
        } else {
            Err(RespError::Incomplete)
        }
    }

    fn parse_integer(buffer: &BytesMut) -> Result<(Value, usize), RespError> {
        if let Some((line, len)) = Self::read_line(&buffer[1..])? {
            let s = String::from_utf8_lossy(line);
            let i = s.parse().map_err(|_| RespError::InvalidProtocol)?;
            Ok((Value::Integer(i), len + 1))
        } else {
            Err(RespError::Incomplete)
        }
    }

    fn parse_bulk_string(buffer: &BytesMut) -> Result<(Value, usize), RespError> {
        if let Some((line, len)) = Self::read_line(&buffer[1..])? {
            let len_str = String::from_utf8_lossy(line);
            let data_len: i64 = len_str.parse().map_err(|_| RespError::InvalidProtocol)?;

            if data_len == -1 {
                return Ok((Value::BulkString(None), len + 1));
            }

            // For now, return simplified implementation
            Ok((Value::BulkString(Some(Bytes::new())), len + 1))
        } else {
            Err(RespError::Incomplete)
        }
    }

    fn parse_array(buffer: &BytesMut) -> Result<(Value, usize), RespError> {
        if let Some((line, len)) = Self::read_line(&buffer[1..])? {
            let len_str = String::from_utf8_lossy(line);
            let num_elements: i64 = len_str.parse().map_err(|_| RespError::InvalidProtocol)?;

            if num_elements == -1 {
                return Ok((Value::Array(None), len + 1));
            }

            // For now, return empty array
            Ok((Value::Array(Some(vec![])), len + 1))
        } else {
            Err(RespError::Incomplete)
        }
    }

    // Correct simpler implementation for parser helpers that just checks completeness

    pub fn decode(src: &mut BytesMut) -> Result<Option<Value>, RespError> {
        if src.is_empty() {
            return Ok(None);
        }

        let mut cursor = std::io::Cursor::new(&src[..]);
        // To implement this fully robustly without `redis-protocol` crate is tedious.
        // I'll implement a simplified synchronous parser that peeks.

        // Simple line reader
        let line_end = match src.windows(2).position(|w| w == b"\r\n") {
            Some(n) => n,
            None => return Ok(None),
        };

        let type_byte = src[0];
        let line_content = &src[1..line_end];
        let next_idx = line_end + 2;

        match type_byte {
            b'+' => {
                let s = String::from_utf8_lossy(line_content).to_string();
                src.advance(next_idx);
                Ok(Some(Value::SimpleString(s)))
            }
            b'-' => {
                let s = String::from_utf8_lossy(line_content).to_string();
                src.advance(next_idx);
                Ok(Some(Value::Error(s)))
            }
            b':' => {
                let s = String::from_utf8_lossy(line_content);
                let i = s.parse().map_err(|_| RespError::InvalidProtocol)?;
                src.advance(next_idx);
                Ok(Some(Value::Integer(i)))
            }
            b'$' => {
                let len_str = String::from_utf8_lossy(line_content);
                let len: i64 = len_str.parse().map_err(|_| RespError::InvalidProtocol)?;

                if len == -1 {
                    src.advance(next_idx);
                    return Ok(Some(Value::BulkString(None)));
                }

                let len = len as usize;
                let total_len = next_idx + len + 2; // +2 for trailing \r\n

                if src.len() < total_len {
                    return Ok(None);
                }

                let data = src[next_idx..next_idx + len].to_vec();
                src.advance(total_len);
                Ok(Some(Value::BulkString(Some(Bytes::from(data)))))
            }
            b'*' => {
                let len_str = String::from_utf8_lossy(line_content);
                let num_elements: i64 = len_str.parse().map_err(|_| RespError::InvalidProtocol)?;

                if num_elements == -1 {
                    src.advance(next_idx);
                    return Ok(Some(Value::Array(None)));
                }

                // We need to consume items.
                // This is recursive. Since we need to modify src, we probably can't easily peek without logic.
                // We'll advance just the header and then loop.
                // BUT, if we run out of data midway, we must rollback or bail.
                // Easier: check sufficient data? No, size of elements varies.
                // We must use a loop that returns None if strictly not enough bytes, WITHOUT advancing.

                // For simplicity in this "stub", I will assume we have the full frame or fail.
                // But for a read loop, we need to be able to tell "wait for more".

                // Let's copy the buffer to test parse. If success, advance original.
                // This is inefficient but safe for prototyping.

                // Better: keep an index.
                let mut current_idx = next_idx;
                let mut items = Vec::new();

                for _ in 0..num_elements {
                    let (val, len) = match Self::peek_parse(&src[current_idx..])? {
                        Some(v) => v,
                        None => return Ok(None),
                    };
                    items.push(val);
                    current_idx += len;
                }

                src.advance(current_idx);
                Ok(Some(Value::Array(Some(items))))
            }
            _ => Err(RespError::InvalidProtocol),
        }
    }

    // Helper to parse without modifying BytesMut, returns (Value, consumed_len)
    fn peek_parse(src: &[u8]) -> Result<Option<(Value, usize)>, RespError> {
        if src.is_empty() {
            return Ok(None);
        }

        let line_end = match src.windows(2).position(|w| w == b"\r\n") {
            Some(n) => n,
            None => return Ok(None),
        };

        let type_byte = src[0];
        let line_content = &src[1..line_end];
        let next_idx = line_end + 2;

        match type_byte {
            b'+' => {
                let s = String::from_utf8_lossy(line_content).to_string();
                Ok(Some((Value::SimpleString(s), next_idx)))
            }
            b'-' => {
                let s = String::from_utf8_lossy(line_content).to_string();
                Ok(Some((Value::Error(s), next_idx)))
            }
            b':' => {
                let s = String::from_utf8_lossy(line_content);
                let i = s.parse().map_err(|_| RespError::InvalidProtocol)?;
                Ok(Some((Value::Integer(i), next_idx)))
            }
            b'$' => {
                let len_str = String::from_utf8_lossy(line_content);
                let len: i64 = len_str.parse().map_err(|_| RespError::InvalidProtocol)?;

                if len == -1 {
                    return Ok(Some((Value::BulkString(None), next_idx)));
                }

                let len = len as usize;
                let total_len = next_idx + len + 2;

                if src.len() < total_len {
                    return Ok(None);
                }

                let data = src[next_idx..next_idx + len].to_vec();
                Ok(Some((
                    Value::BulkString(Some(Bytes::from(data))),
                    total_len,
                )))
            }
            b'*' => {
                let len_str = String::from_utf8_lossy(line_content);
                let num_elements: i64 = len_str.parse().map_err(|_| RespError::InvalidProtocol)?;

                if num_elements == -1 {
                    return Ok(Some((Value::Array(None), next_idx)));
                }

                let mut current_idx = next_idx;
                let mut items = Vec::new();

                for _ in 0..num_elements {
                    let (val, len) = match Self::peek_parse(&src[current_idx..])? {
                        Some(v) => v,
                        None => return Ok(None),
                    };
                    items.push(val);
                    current_idx += len;
                }
                Ok(Some((Value::Array(Some(items)), current_idx)))
            }
            _ => Err(RespError::InvalidProtocol),
        }
    }
}

// Extension to BytesMut for advance
trait BytesAdvance {
    fn advance(&mut self, cnt: usize);
}
impl BytesAdvance for BytesMut {
    fn advance(&mut self, cnt: usize) {
        let _ = self.split_to(cnt);
    }
}
