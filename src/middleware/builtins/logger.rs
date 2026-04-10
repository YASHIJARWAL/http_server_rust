use std::time::Instant;

use crate::{http::{request::Request, response::Response}, middleware::{Middleware, Next}};

pub struct Logger;
impl Middleware for Logger{
    fn handle(&self, req : &Request,next :Next)->Response {
        let start = Instant::now();
        let response = next.run(req);
        println!("INFO {}{} completed in {:?}",req.method,req.path,start.elapsed());
        response
    }
}