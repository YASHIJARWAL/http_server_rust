mod thread_pool;
mod http;
mod static_files;
mod routes;
mod middleware;
use thread_pool::ThreadPool;
use crate::http::response::Response;
use crate::middleware::logger;
use crate::static_files::file_server::serve_file;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::sync::Arc;
use http::request::Request;
use crate::routes::handler::{hello_handler,user_handler};
use crate::routes::router::Router;

fn process_request(request: &Request,router:&Router)->Response{
    let response = router.handle(&request);
    if response.status_code == 404 {
        if let Some(file_response) = serve_file(&request.path) {
            return file_response;
        }
        if let Some(mut not_found_page)=serve_file("/error404.html"){
                not_found_page.status_code=404;
                return not_found_page;
        }
        return Response::new(404).headers("content-type", "text/html").body("404:not found".as_bytes());
    }
    response
            
}
fn handle_connection(mut stream: TcpStream,router:Arc<Router>) {

    println!("Connection received");

    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    println!("Bytes read: {}", bytes_read);

    let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request = Request::parse(&request_string);
    println!("method:{} path:{} body:{}",request.method,request.path,request.body);
     let response = logger::log_request(
        &request.method,
        &request.path,
        || {match std::panic::catch_unwind(||process_request(&request, &router)){
            Ok(resp)=>resp,
            Err(_)=>Response::new(500).headers("content-type", "text/plain").body("500 Internal Server Error".as_bytes()),
        }
    }
    );
    let response_bytes = response.to_http_bytes();
    stream.write_all(&response_bytes).unwrap();
}
fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    println!("server is running in port 7878");
    let mut router = Router::new();
    router.get("/hello",hello_handler);
    router.get("/users",user_handler);
    let router =Arc::new(router);
    for stream in listener.incoming(){

        let stream = stream.unwrap();
        let router =Arc::clone(&router);

        pool.execute(||{
            handle_connection(stream,router);
        });
    }
}