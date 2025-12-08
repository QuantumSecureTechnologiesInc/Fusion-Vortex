// src/web/mod.rs - Web Framework for Fusion
// Provides HTTP server, routing, and middleware support

pub mod http;
pub mod router;
pub mod server;

/// Web Framework Error
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum WebError {
    BindError(String),
    RouteNotFound,
    InternalServerError(String),
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::BindError(msg) => write!(f, "Bind error: {}", msg),
            WebError::RouteNotFound => write!(f, "Route not found"),
            WebError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}
