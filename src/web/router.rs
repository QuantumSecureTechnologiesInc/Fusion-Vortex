// src/web/router.rs - Request Router
#![allow(dead_code)]

use super::http::{Method, Request, Response};
use std::collections::HashMap;

type Handler = Box<dyn Fn(Request) -> Response + Send + Sync>;

pub struct Router {
    routes: HashMap<(Method, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .insert((method, path.to_string()), Box::new(handler));
    }

    pub fn handle(&self, req: Request) -> Response {
        if let Some(handler) = self.routes.get(&(req.method.clone(), req.path.clone())) {
            handler(req)
        } else {
            Response::not_found()
        }
    }
}
