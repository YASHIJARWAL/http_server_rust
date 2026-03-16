use std::fs;
use std::path::Path;
use crate::http::response::Response;
fn get_content_type(path: &str) -> &str {
    if path.ends_with(".html") {
        "text/html"
    }
    else if path.ends_with(".css") {
        "text/css"
    }
    else if path.ends_with(".js") {
        "application/javascript"
    }
    else if path.ends_with(".json") {
        "application/json"
    }
    
    else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    }
    else if path.ends_with(".png") {
    "image/png"
    }
    else if path.ends_with(".ico") {
        "image/x-icon"
    }
    else if path.ends_with(".gif") {
        "image/gif"
    }
    else {
        "text/plain"
    }
}

pub fn serve_file(path:&str)->Option<Response>{
    let file_path=format!("public{}",path);
    if !Path::new(&file_path).exists(){
        return None;
    }
    let content=fs::read(&file_path).ok()?;
    Some(Response::new(200).headers("content-type", get_content_type(path)).body(&content))
}