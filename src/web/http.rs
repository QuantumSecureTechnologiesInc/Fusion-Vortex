// src/web/http.rs - HTTP Types
#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(method: Method, path: String, body: Vec<u8>) -> Self {
        let (clean_path, query) = Self::parse_path(&path);

        Self {
            method,
            path: clean_path,
            query_params: query,
            headers: HashMap::new(),
            body,
        }
    }

    fn parse_path(raw_path: &str) -> (String, HashMap<String, String>) {
        if let Some((path, query_str)) = raw_path.split_once('?') {
            let mut params = HashMap::new();
            for part in query_str.split('&') {
                if let Some((k, v)) = part.split_once('=') {
                    params.insert(k.to_string(), v.to_string());
                } else if !part.is_empty() {
                    params.insert(part.to_string(), String::new());
                }
            }
            (path.to_string(), params)
        } else {
            (raw_path.to_string(), HashMap::new())
        }
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn json<T: serde::Serialize>(data: &T) -> Self {
        let json = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        Self {
            status: 200,
            headers,
            body: json.into_bytes(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            headers: HashMap::new(),
            body: b"Not Found".to_vec(),
        }
    }

    pub fn internal_server_error(msg: &str) -> Self {
        Self {
            status: 500,
            headers: HashMap::new(),
            body: msg.to_string().into_bytes(),
        }
    }
}
