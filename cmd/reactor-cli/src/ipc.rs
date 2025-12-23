// reactor_cli/src/ipc.rs
// Production IPC for Cluster Context

#[cfg(not(unix))]
use std::net::TcpStream as UnixStream;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

use serde_json::Value;
use std::io::{Read, Write};

pub struct IpcClient {
    socket_path: String,
}

impl IpcClient {
    pub fn new() -> Self {
        Self {
            socket_path: "/var/run/fusion/fusion.sock".to_string(),
        }
    }

    pub fn query_context(&self, key: &str) -> Option<String> {
        // 1. Connect to Daemon
        let mut stream = match UnixStream::connect(&self.socket_path) {
            Ok(s) => s,
            Err(_) => return None,
        };

        // 2. Send Request
        let request = serde_json::json!({
            "op": "GET_CONTEXT",
            "key": key
        });

        if stream.write_all(request.to_string().as_bytes()).is_err() {
            return None;
        }

        // 3. Read Response with proper framing
        let mut response = String::new();
        if stream.read_to_string(&mut response).is_err() {
            return None;
        }

        let v: Value = serde_json::from_str(&response).ok()?;
        v.get("value")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    }
}

impl Default for IpcClient {
    fn default() -> Self {
        Self::new()
    }
}
