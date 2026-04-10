
use crate::http::request::Request;
use crate::http::response::Response;
pub fn hello_handler(_req: &Request)->Response{
    Response::new(200).headers("Content-type", "text/plain").body("hello from rust router".as_bytes())
}
pub fn user_handler(_req: &Request)->Response{
    Response::new(200).headers("Content-Type","application/json").body(r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#.as_bytes())
}
pub fn create_user_handler(req:&Request)->Response{
    let body = format!("Recieve Post data: \n{}",req.body);
    Response::new(200).headers("content-type", "text/plain").body(body.as_bytes())
}