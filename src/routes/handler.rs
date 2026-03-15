use crate::http::request::Request;
use crate::http::response::Response;
pub fn hello_handler(_req: &Request)->Response{
    Response::new(200).headers("Content-type", "text/plain").body("hello from rust router")
}
pub fn user_handler(_req: &Request)->Response{
    Response::new(200).headers("Content-Type","application/json").body(r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#)
}
