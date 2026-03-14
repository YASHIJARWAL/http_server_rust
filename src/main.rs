use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\nHello World";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("server is running in port 7878");
    for stream in listner.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
