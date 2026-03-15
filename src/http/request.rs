use std::{collections::HashMap};

#[allow(dead_code)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String,String>,
    pub body:String,
}
impl Request {

    pub fn parse(request: &str) -> Request {

        let mut lines = request.lines();

        let request_line = lines.next().unwrap();

        let parts: Vec<&str> = request_line.split_whitespace().collect();

        let method = parts[0].to_string();
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        let mut headers = HashMap::new();

        for line in &mut lines {

            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.to_string(), value.to_string());
            }
        }
        let body: String = lines.collect::<Vec<&str>>().join("\n");
        Request {
            method,
            path,
            version,
            headers,
            body,
        }
    }
}