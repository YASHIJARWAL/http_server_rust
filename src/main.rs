mod thread_pool;
mod http;
mod static_files;
mod routes;
mod middleware;
use thread_pool::ThreadPool;
use crate::middleware::logger;
use crate::static_files::file_server::serve_file;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::sync::Arc;
use http::request::Request;
use crate::routes::handler::{hello_handler,user_handler};
use crate::routes::router::Router;

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
        || {
            let mut response = router.handle(&request);
            if response.status_code == 404 {
                if let Some(file_response) = serve_file(&request.path) {
                    response = file_response;
                }
                else{
                    if let Some(not_found_page)=serve_file("/error404.html"){
                        response=not_found_page;
                    }
                }
            }
            response
        }
    );
    let response_string = response.to_http_string();
    stream.write_all(response_string.as_bytes()).unwrap();
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