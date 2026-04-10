use std::collections::HashMap;

use crate::http::{request::Request, response::Response};
use crate::middleware::{Middleware, Next};

pub type Handler = fn(&Request) -> Response;

#[derive(Hash, Eq, PartialEq)]
struct RouteKey {
    method: String,
    path: String,
}

pub struct Router {
    routes: HashMap<RouteKey, Handler>,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route("GET", path, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route("POST", path, handler);
    }

    pub fn add_route(&mut self, method: &str, path: &str, handler: Handler) {
        let key = RouteKey {
            method: method.to_string(),
            path: path.to_string(),
        };

        self.routes.insert(key, handler);
    }

    
    pub fn use_middleware<M: Middleware + 'static>(&mut self, m: M) {
        self.middlewares.push(Box::new(m));
    }

    
    pub fn handle(&self, request: &Request) -> Response {
        let key = RouteKey {
            method: request.method.clone(),
            path: request.path.clone(),
        };

        let handler = self.routes.get(&key).copied();

        let final_handler: Handler = match handler {
            Some(h) => h,
            None => Self::not_found,
        };

        let next = Next {
            middlewares: &self.middlewares,
            handler: final_handler,
        };

        next.run(request)
    }

    fn not_found(_req: &Request) -> Response {
        Response::new(404)
            .headers("Content-Type", "text/plain")
            .body("Route Not Found".as_bytes())
    }
}