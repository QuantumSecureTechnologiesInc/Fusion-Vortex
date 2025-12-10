// src/network/rpc.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// Execute a remote function with arguments
    Execute {
        function_name: String,
        args: Vec<u8>,
    },

    /// Return the result of an execution
    Result { success: bool, data: Vec<u8> },

    /// Health check / keepalive
    Ping,

    /// Response to Ping
    Pong,

    /// Authenticate client with token/secret
    Authenticate { client_id: String, token: String },

    /// Authentication response
    AuthResult { success: bool, message: String },
}
