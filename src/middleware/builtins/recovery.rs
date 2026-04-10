use std::panic::AssertUnwindSafe;

use crate::{http::{request::Request, response::Response}, middleware::{Middleware, Next}};

pub struct Recovery;

impl Middleware for Recovery {
    fn handle(&self, req: &Request, next: Next) -> Response {
        match std::panic::catch_unwind(AssertUnwindSafe(|| next.run(req))) {
            Ok(resp) => resp,
            Err(_) => Response::new(500)
                .headers("Content-Type", "text/plain")
                .body("Internal Server Error".as_bytes()),
        }
    }
}