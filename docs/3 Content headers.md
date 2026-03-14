# Step 4 — HTTP Headers and JSON API Responses

## Overview

In this step, the Rust TCP server was upgraded to return **proper HTTP responses with headers** and support **basic API-style routing**.

Previously, the server always returned a simple text response without headers.
Now the server behaves more like a real backend API by including HTTP metadata and returning structured responses such as JSON.

This step introduces key backend concepts:

* HTTP response structure
* Response headers
* Content types
* Basic API endpoints

---

# What Changed From the Previous Step

Earlier response:

```
HTTP/1.1 200 OK

Hello World
```

Updated response:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 22
Connection: close

Hello from Rust server
```

The server now includes **standard HTTP headers**, which are required for proper browser and API client behavior.

---

# Implemented Routes

The server now supports three routes.

| Route                 | Description              | Response Type |
| --------------------- | ------------------------ | ------------- |
| `/hello`              | Simple greeting endpoint | Text          |
| `/users`              | Returns user data        | JSON          |
| `/` or any other path | Default welcome message  | Text          |

---

# Example Requests

### Request

```
GET /hello HTTP/1.1
Host: 127.0.0.1:7878
```

Response:

```
Hello from Rust server
```

---

### Request

```
GET /users HTTP/1.1
Host: 127.0.0.1:7878
```

Response:

```
[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]
```

This response uses the content type:

```
Content-Type: application/json
```

which tells the client the response body contains JSON data.

---

# Important HTTP Headers Added

### Content-Type

Defines the format of the response body.

Examples used:

```
text/plain
application/json
```

---

### Content-Length

Indicates the size of the response body in bytes.

Example:

```
Content-Length: 37
```

Clients use this header to determine when the response body has fully arrived.

---

### Connection: close

```
Connection: close
```

This header instructs the client to close the TCP connection after the response is sent.

Since the current server implementation does not support persistent connections, this prevents hanging connections.

---

# Reading the HTTP Request

Incoming requests are read from the TCP stream as **raw bytes**.

```
stream.read(&mut buffer)
```

The number of bytes read is stored in:

```
bytes_read
```

The request is then converted to text using:

```
String::from_utf8_lossy(&buffer[..bytes_read])
```

This converts the raw network bytes into a readable string so the server can inspect the request.

---

# Routing Logic

Routing is currently implemented using simple string matching.

Example:

```
request.starts_with("GET /hello")
```

This allows the server to detect which endpoint the client requested.

Although basic, this demonstrates the **core concept behind routing engines used in web frameworks**.

---

# Complete Implementation

```rust
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let (status_line, body, content_type) =
        if request.starts_with("GET /hello") {

            ("HTTP/1.1 200 OK",
            "Hello from Rust server",
            "text/plain")

        } else if request.starts_with("GET /users") {

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

    println!("server is running on port 7878");

    for stream in listener.incoming(){

        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
```

---

# What This Step Demonstrates

This stage introduces fundamental backend engineering concepts:

* Manual HTTP response construction
* HTTP headers
* JSON API responses
* Basic route matching

These ideas form the foundation of modern backend frameworks.

---

# Next Step

The next improvement will introduce **concurrency**.

Currently the server processes requests sequentially:

```
Client 1 → handled
Client 2 → waits
Client 3 → waits
```

In the next step we will implement a **Thread Pool**, allowing multiple client requests to be processed simultaneously.

This is a core systems programming concept used by production web servers.
