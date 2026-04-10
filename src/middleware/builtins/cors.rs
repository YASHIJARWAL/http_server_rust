use crate::{http::{request::Request, response::Response}, middleware::{Middleware, Next}};
pub struct CORS;
impl Middleware for CORS {
    fn handle(&self, req : &Request,next :Next)->Response {
        let mut response = next.run(req);
        response.headers.insert("access-control-allow-origin".to_string(), "*".to_string());
        response
    }
}