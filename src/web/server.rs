// src/web/server.rs - HTTP Server Implementation
#![allow(dead_code)]

use super::router::Router;
use super::WebError;

pub struct Server {
    router: Router,
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            router: Router::new(),
            port,
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(super::http::Request) -> super::http::Response + Send + Sync + 'static,
    {
        self.router
            .add_route(super::http::Method::GET, path, handler);
    }

    pub fn start(&self) -> Result<(), WebError> {
        println!("Starting Web Server on port {}", self.port);
        // In a real implementation:
        // 1. Bind TCP listener
        // 2. Loop accept connections
        // 3. Parse HTTP request
        // 4. Delegate to self.router.handle(req)
        Ok(())
    }
}
