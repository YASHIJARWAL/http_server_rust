use std::{collections::HashMap};

pub struct Response{
    pub status_code:u16,
    pub headers: HashMap<String,String>,
    pub body:String,
}
impl Response{
    pub fn new(status_code:u16)->Self{
        Response{status_code, headers:HashMap::new(), body:String::new()}
    }
    pub fn headers(mut self,key:&str, value:&str)->Self{
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    pub fn body(mut self, body:&str)->Self{
        self.body=body.to_string();
        self
    }
    pub fn to_http_string(&self) -> String {

        let mut response = format!("HTTP/1.1 {} OK\r\n", self.status_code);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}