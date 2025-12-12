/// SafeTensors Parser.
/// 
/// Implements the logic to read the header size and parse the JSON metadata
/// from a SafeTensors file, which is safer and faster than Pickle/PyTorch formats.

use fusion_std::error::{StdResult, StdError};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use serde_json::Value;

pub struct SafeTensorsMetadata {
    pub tensors: std::collections::HashMap<String, TensorInfo>,
    pub header_size: u64,
}

#[derive(Debug)]
pub struct TensorInfo {
    pub dtype: String,
    pub shape: Vec<usize>,
    pub data_offsets: (u64, u64),
}

pub struct SafeTensorsParser;

impl SafeTensorsParser {
    /// Parse the header of a SafeTensors byte stream.
    /// Format: [u64 length] [JSON header bytes] [Data...]
    pub fn parse_header(data: &[u8]) -> StdResult<SafeTensorsMetadata> {
        let mut cursor = Cursor::new(data);
        
        // 1. Read Header Size (8 bytes, Little Endian)
        let header_len = cursor.read_u64::<LittleEndian>()
            .map_err(|e| StdError::Serialization(format!("Failed to read header size: {}", e)))?;
            
        // Safety check on size (avoid OOM on malformed files)
        if header_len as usize > 100 * 1024 * 1024 { // 100MB limit
            return Err(StdError::Serialization("Header size too large".into()));
        }
        
        // 2. Read JSON Header
        let start = 8;
        let end = start + header_len as usize;
        
        if end > data.len() {
            return Err(StdError::Serialization("Header length exceeds file size".into()));
        }
        
        let header_bytes = &data[start..end];
        let header_json: Value = serde_json::from_slice(header_bytes)
            .map_err(|e| StdError::Serialization(format!("Invalid JSON header: {}", e)))?;
            
        // 3. Parse Tensor Info
        let mut tensor_map = std::collections::HashMap::new();
        
        if let Some(obj) = header_json.as_object() {
            for (key, val) in obj {
                // Skip metadata keys (usually starts with __)
                if key.starts_with("__") { continue; }
                
                let dtype = val["dtype"].as_str().ok_or(StdError::Serialization("Missing dtype".into()))?.to_string();
                
                let shape_arr = val["shape"].as_array().ok_or(StdError::Serialization("Missing shape".into()))?;
                let shape: Vec<usize> = shape_arr.iter().map(|v| v.as_u64().unwrap() as usize).collect();
                
                let offsets = val["data_offsets"].as_array().ok_or(StdError::Serialization("Missing offsets".into()))?;
                let start_off = offsets[0].as_u64().unwrap();
                let end_off = offsets[1].as_u64().unwrap();
                
                tensor_map.insert(key.clone(), TensorInfo {
                    dtype,
                    shape,
                    data_offsets: (start_off, end_off),
                });
            }
        }
        
        Ok(SafeTensorsMetadata {
            tensors: tensor_map,
            header_size: header_len,
        })
    }
}