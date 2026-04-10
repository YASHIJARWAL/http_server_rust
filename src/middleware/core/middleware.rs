use crate::{http::{request::Request, response::Response}, middleware::Next};

pub trait Middleware: Send + Sync {
    fn handle(&self, req : &Request,next :Next)->Response;
}