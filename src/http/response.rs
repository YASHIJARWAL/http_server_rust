use std::{collections::HashMap,};

pub struct Response{
    pub status_code:u16,
    pub headers: HashMap<String,String>,
    pub body:Vec<u8>,
}
impl Response{
    pub fn new(status_code:u16)->Self{
        Response{status_code, headers:HashMap::new(), body:Vec::new()}
    }
    pub fn headers(mut self,key:&str, value:&str)->Self{
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    pub fn body(mut self, body:&[u8])->Self{
        self.body=body.to_vec();
        self
    }
    pub fn status_text(code: u16) -> &'static str {
        match code {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "OK",
        }
    }
    pub fn to_http_bytes(&self) -> Vec<u8> {

        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            Self::status_text(self.status_code)
        );

        let mut response = status_line;

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend(&self.body);

        bytes
    }
}