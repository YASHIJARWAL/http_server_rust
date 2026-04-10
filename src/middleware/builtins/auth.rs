use crate::{http::{request::Request, response::Response}, middleware::{Middleware, Next}};

pub struct Auth;
impl Middleware for Auth{
    fn handle(&self, req :&Request,next :Next)->Response {
        if let Some(token)=req.headers.get("Authorization"){
            if token=="secret"{
                return next.run(req);
            }
        }
        Response::new(401).headers("content-type", "text/plain").body("Unauthorized".as_bytes())
    }
}