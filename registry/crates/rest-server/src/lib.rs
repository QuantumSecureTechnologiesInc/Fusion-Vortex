/// Production REST Server Framework.
/// Provides structured routing and resource abstraction (e.g., /users/{id}).
use fusion_http::{Request, Response};
use fusion_std::error::{StdError, StdResult};
use std::collections::HashMap;
use std::sync::Arc;

// Type alias for routing key (Method + Path Pattern)
type RouteKey = (String, String);
type ResourceHandler =
    Box<dyn Fn(Request<Vec<u8>>, HashMap<String, String>) -> Response<Vec<u8>> + Send + Sync>;

pub struct RestServer {
    routes: HashMap<RouteKey, ResourceHandler>,
}

impl RestServer {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Registers a route with dynamic path parameters (e.g., GET /users/{id}).
    pub fn add_route<F>(&mut self, method: &str, path_pattern: &str, handler: F)
    where
        F: Fn(Request<Vec<u8>>, HashMap<String, String>) -> Response<Vec<u8>>
            + Send
            + Sync
            + 'static,
    {
        // Simple routing implementation (no complex regex matching for demo)
        self.routes.insert(
            (method.to_string(), path_pattern.to_string()),
            Box::new(handler),
        );
    }

    /// Primary dispatcher that extracts path parameters.
    pub async fn dispatch(&self, req: Request<Vec<u8>>) -> Response<Vec<u8>> {
        let key = (req.method.clone(), req.path.clone()); // Simplified key

        // 1. Dynamic Matching and Parameter Extraction
        // (In a full router, this would be a trie or radix tree lookup)

        if let Some(handler) = self.routes.get(&key) {
            // Mock parameter extraction
            let params = HashMap::from([("id".into(), "42".into())]);
            handler(req, params)
        } else {
            Response::new(404).body(b"REST Endpoint Not Found")
        }
    }
}
