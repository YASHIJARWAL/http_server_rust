mod thread_pool;
mod http;
use thread_pool::ThreadPool;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use http::request::Request;

fn handle_connection(mut stream: TcpStream) {

    println!("Connection received");

    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    println!("Bytes read: {}", bytes_read);

    let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request = Request::parse(&request_string);
    println!("method:{},path:{}",request.method,request.path);

    let (status_line, body, content_type) =
        if request.path=="/hello" {

            ("HTTP/1.1 200 OK",
            "Hello from Rust server",
            "text/plain")

        } else if request.path=="/users" {

            ("HTTP/1.1 200 OK",
            r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#,
            "application/json")

        } else {

            ("HTTP/1.1 200 OK",
            "Welcome to the server",
            "text/plain")
        };

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_line,
        content_type,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).unwrap();
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