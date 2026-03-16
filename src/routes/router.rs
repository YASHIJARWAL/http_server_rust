use std::collections::HashMap;

use crate::http::response::Response;
use crate::http::request::Request;
pub type Handler = fn(&Request)->Response;
pub struct Router{
    routes:HashMap<String,Handler>,
}
impl Router {
    pub fn new()->Self{
        Router { 
            routes:HashMap::new() 
        }
    }
    pub fn get(&mut self , path:&str , handler: Handler){
        self.routes.insert(path.to_string(), handler);
    }
    pub fn handle(&self,request:&Request)->Response{
        if let Some(handler)=self.routes.get(&request.path){
            handler(request)
        }
        else{
            Response::new(404).headers("Content-type", "text/plain").body("routes not found".as_bytes())
        }
    }
}