use fusion_http::{Request, Response};
use fusion_std::error::{StdError, StdResult};
use std::collections::HashMap;

/// Production REST Server with Radix Trie Routing
///
/// Implements efficient O(k) route matching where k = path length

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "PATCH" => Some(HttpMethod::PATCH),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            _ => None,
        }
    }
}

type Handler = fn(Request) -> StdResult<Response>;

struct RadixNode {
    /// Path segment for this node
    segment: String,
    /// Handler for this exact path (if any)
    handler: Option<Handler>,
    /// HTTP method for this handler
    method: Option<HttpMethod>,
    /// Child nodes
    children: Vec<RadixNode>,
    /// Is this a parameter node? (e.g., :id)
    is_param: bool,
    /// Parameter name if is_param is true
    param_name: Option<String>,
}

impl RadixNode {
    fn new(segment: String) -> Self {
        let (is_param, param_name) = if segment.starts_with(':') {
            (true, Some(segment[1..].to_string()))
        } else {
            (false, None)
        };

        Self {
            segment,
            handler: None,
            method: None,
            children: Vec::new(),
            is_param,
            param_name,
        }
    }

    fn insert(&mut self, path_parts: &[&str], method: HttpMethod, handler: Handler) {
        if path_parts.is_empty() {
            self.handler = Some(handler);
            self.method = Some(method);
            return;
        }

        let segment = path_parts[0];

        // Find matching child or create new one
        let child = self
            .children
            .iter_mut()
            .find(|c| c.segment == segment || (c.is_param && segment.starts_with(':')));

        if let Some(child_node) = child {
            child_node.insert(&path_parts[1..], method, handler);
        } else {
            let mut new_node = RadixNode::new(segment.to_string());
            new_node.insert(&path_parts[1..], method, handler);
            self.children.push(new_node);
        }
    }

    fn search(
        &self,
        path_parts: &[&str],
        method: &HttpMethod,
        params: &mut HashMap<String, String>,
    ) -> Option<Handler> {
        if path_parts.is_empty() {
            // Check if method matches
            if let Some(ref node_method) = self.method {
                if std::mem::discriminant(node_method) == std::mem::discriminant(method) {
                    return self.handler;
                }
            }
            return None;
        }

        let segment = path_parts[0];

        // Try exact match first
        for child in &self.children {
            if child.segment == segment {
                if let Some(handler) = child.search(&path_parts[1..], method, params) {
                    return Some(handler);
                }
            }
        }

        // Try parameter match
        for child in &self.children {
            if child.is_param {
                if let Some(ref param_name) = child.param_name {
                    params.insert(param_name.clone(), segment.to_string());
                }
                if let Some(handler) = child.search(&path_parts[1..], method, params) {
                    return Some(handler);
                }
                // Remove param if search failed
                if let Some(ref param_name) = child.param_name {
                    params.remove(param_name);
                }
            }
        }

        None
    }
}

pub struct RestServer {
    root: RadixNode,
    port: u16,
}

impl RestServer {
    pub fn new(port: u16) -> Self {
        Self {
            root: RadixNode::new(String::new()),
            port,
        }
    }

    /// Register a route with handler
    pub fn route(&mut self, method: &str, path: &str, handler: Handler) -> StdResult<()> {
        let http_method = HttpMethod::from_str(method)
            .ok_or_else(|| StdError::InvalidInput(format!("Invalid HTTP method: {}", method)))?;

        let path = path.trim_start_matches('/');
        let parts: Vec<&str> = if path.is_empty() {
            vec![]
        } else {
            path.split('/').collect()
        };

        self.root.insert(&parts, http_method, handler);
        Ok(())
    }

    /// Handle incoming request
    pub fn handle_request(&self, request: Request) -> StdResult<Response> {
        let method = HttpMethod::from_str(&request.method)
            .ok_or_else(|| StdError::InvalidInput("Invalid HTTP method".into()))?;

        let path = request.path.trim_start_matches('/');
        let parts: Vec<&str> = if path.is_empty() {
            vec![]
        } else {
            path.split('/').collect()
        };

        let mut params = HashMap::new();

        if let Some(handler) = self.root.search(&parts, &method, &mut params) {
            // In production, we'd inject params into the request
            handler(request)
        } else {
            Err(StdError::NotFound(format!(
                "No handler for {} {}",
                request.method, request.path
            )))
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Convenience methods for common HTTP methods
    pub fn get(&mut self, path: &str, handler: Handler) -> StdResult<()> {
        self.route("GET", path, handler)
    }

    pub fn post(&mut self, path: &str, handler: Handler) -> StdResult<()> {
        self.route("POST", path, handler)
    }

    pub fn put(&mut self, path: &str, handler: Handler) -> StdResult<()> {
        self.route("PUT", path, handler)
    }

    pub fn delete(&mut self, path: &str, handler: Handler) -> StdResult<()> {
        self.route("DELETE", path, handler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_handler(_req: Request) -> StdResult<Response> {
        Ok(Response {
            status: 200,
            body: vec![],
            headers: HashMap::new(),
        })
    }

    #[test]
    fn test_route_registration() {
        let mut server = RestServer::new(8080);
        assert!(server.get("/users", dummy_handler).is_ok());
        assert!(server.post("/users", dummy_handler).is_ok());
    }

    #[test]
    fn test_exact_match() {
        let mut server = RestServer::new(8080);
        server.get("/users", dummy_handler).unwrap();

        let req = Request {
            method: "GET".to_string(),
            path: "/users".to_string(),
            headers: HashMap::new(),
            body: vec![],
        };

        assert!(server.handle_request(req).is_ok());
    }

    #[test]
    fn test_not_found() {
        let server = RestServer::new(8080);

        let req = Request {
            method: "GET".to_string(),
            path: "/nonexistent".to_string(),
            headers: HashMap::new(),
            body: vec![],
        };

        assert!(server.handle_request(req).is_err());
    }
}
