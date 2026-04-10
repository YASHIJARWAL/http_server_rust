use std::sync::{Mutex};
use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::http::request::Request;
use crate::http::response::Response;
use crate::middleware::{Middleware, Next};

pub struct RateLimiter {
    store: Mutex<HashMap<String, (u32, Instant)>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Middleware for RateLimiter {
    fn handle(&self, req: &Request, next: Next) -> Response {
        let mut store = self.store.lock().unwrap();
        let entry = store.entry(req.path.clone()).or_insert((0, Instant::now()));

        if entry.1.elapsed() > Duration::from_secs(60) {
            *entry = (0, Instant::now());
        }

        entry.0 += 1;

        if entry.0 > 10 {
            return Response::new(429)
                .headers("Content-Type", "text/plain")
                .body("Too Many Requests".as_bytes());
        }

        next.run(req)
    }
}