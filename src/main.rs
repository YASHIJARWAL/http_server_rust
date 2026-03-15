mod thread_pool;
mod http;
use thread_pool::ThreadPool;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use http::request::Request;
use http::response::Response;

fn handle_connection(mut stream: TcpStream) {

    println!("Connection received");

    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    println!("Bytes read: {}", bytes_read);

    let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request = Request::parse(&request_string);
    println!("method:{} path:{} body:{}",request.method,request.path,request.body);
    let response =
        if request.path=="/hello" {
            Response::new(200).headers("content-type","text/plain").body("hello from RUST Server")

        } 
        else if request.path=="/users" {

            Response::new(200).headers("content-type","application/json").body(r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#)

        } else {
            Response::new(200).headers("content-type","text/plain").body("Welcome to RUST Server")

        };

    let response_string = response.to_http_string();

    stream.write_all(response_string.as_bytes()).unwrap();
}
fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("server is running in port 7878");

    for stream in listener.incoming(){

        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });
    }
}