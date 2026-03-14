# Minimal HTTP Server in Rust

## Overview

This project implements a **very basic HTTP server** using only Rust's standard library. The server listens for incoming TCP connections, reads the HTTP request sent by a client (such as a web browser), and responds with a simple `Hello World` message.

The goal of this project is educational: to understand the **fundamental mechanics behind web servers and backend frameworks**.

---

# How the Server Works

The server performs four main steps:

1. Open a TCP socket and listen on a port.
2. Accept incoming connections.
3. Read the HTTP request sent by the client.
4. Send an HTTP response back to the client.

---

# Source Code

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\nHello World";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server running on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
```

---

# Code Explanation

## Imports

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
```

These imports bring networking and input/output capabilities from Rust's standard library.

* `TcpListener` — Listens for incoming TCP connections.
* `TcpStream` — Represents an active connection between client and server.
* `Read` — Allows reading bytes from a stream.
* `Write` — Allows sending bytes to a stream.

---

## Handling a Client Connection

```rust
fn handle_connection(mut stream: TcpStream)
```

This function handles one incoming client connection.

Each browser request results in a new `TcpStream` connection that is passed to this function.

---

## Request Buffer

```rust
let mut buffer = [0; 1024];
```

A fixed-size buffer is created to store the incoming request.

The server reads up to **1024 bytes** from the client request.

---

## Reading the HTTP Request

```rust
stream.read(&mut buffer).unwrap();
```

The server reads raw bytes from the TCP connection into the buffer.

These bytes contain the HTTP request sent by the browser.

Example request:

```
GET / HTTP/1.1
Host: localhost:7878
User-Agent: Mozilla/5.0
Accept: */*
```

Currently the server **does not parse the request**; it only reads it.

---

## Preparing the HTTP Response

```rust
let response = "HTTP/1.1 200 OK\r\n\r\nHello World";
```

This string represents a valid HTTP response.

Structure:

```
HTTP/1.1 200 OK
(blank line)
Hello World
```

The sequence `\r\n\r\n` separates HTTP headers from the response body.

---

## Sending the Response

```rust
stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

The server sends the response back to the client through the TCP stream.

* `write()` sends the bytes.
* `flush()` ensures the data is transmitted immediately.

---

## Starting the Server

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
```

This line creates a TCP server bound to:

```
127.0.0.1:7878
```

Meaning:

* `127.0.0.1` → localhost
* `7878` → port number

---

## Accepting Incoming Connections

```rust
for stream in listener.incoming() {
```

This loop continuously waits for new client connections.

Every time a client connects, a new `TcpStream` is created.

---

## Passing the Connection to the Handler

```rust
let stream = stream.unwrap();
handle_connection(stream);
```

Each connection is processed by the `handle_connection` function.

The server handles **one connection at a time**.

---

# Running the Server

### Build and run the program

```
cargo run
```

Expected output:

```
Server running on port 7878
```

---

# Testing the Server

Open a browser and visit:

```
http://127.0.0.1:7878
```

The browser will display:

```
Hello World
```

---

# Limitations of the Current Server

This server is intentionally minimal and has several limitations:

* No routing
* No HTTP request parsing
* No headers
* No JSON responses
* No concurrency
* No error handling

Every request returns the same `Hello World` response.

---

# Possible Improvements

Future enhancements could include:

### 1. Request Parsing

Extract HTTP method, headers, and path.

### 2. Routing

Support different endpoints such as:

```
/hello
/users
/products
```

### 3. Concurrency

Handle multiple requests simultaneously using threads or async execution.

### 4. HTTP Headers

Return proper headers like `Content-Type`.

### 5. Logging

Log incoming requests and responses.

### 6. JSON Responses

Allow structured API responses.

---

# Learning Outcomes

This project demonstrates the core concepts behind web servers:

* TCP networking
* HTTP request/response structure
* socket communication
* server loops

These ideas form the foundation of modern backend frameworks.

---

# Summary

You have built a minimal HTTP server capable of:

* Listening for TCP connections
* Receiving HTTP requests
* Sending valid HTTP responses

Although simple, this program represents the fundamental building block behind full-featured web frameworks and backend systems.
