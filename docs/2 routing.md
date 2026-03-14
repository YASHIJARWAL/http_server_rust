# Update: Basic HTTP Request Parsing and Routing

## Overview

In this update, the server was improved to **inspect incoming HTTP requests** and return different responses based on the requested path.
Previously, the server ignored the request and always returned the same response.

This update introduces two important backend concepts:

* **Reading and interpreting HTTP requests**
* **Basic routing based on request paths**

---

# Previous Behavior

The original server logic performed the following steps:

1. Accept a TCP connection
2. Read bytes from the connection
3. Ignore the request contents
4. Return a fixed HTTP response

Example response for any request:

```
HTTP/1.1 200 OK

Hello World
```

Regardless of the browser URL (`/`, `/hello`, `/users`), the response remained the same.

---

# New Behavior: Route-Based Responses

The server now inspects the request and sends different responses depending on the requested path.

Supported routes:

| Route    | Response               |
| -------- | ---------------------- |
| `/hello` | Hello from Rust server |
| `/users` | User list page         |
| `/`      | Welcome to the server  |

---

# Updated Code

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);

    let response = if request.starts_with("GET /hello") {
        "HTTP/1.1 200 OK\r\n\r\nHello from Rust server"
    } 
    else if request.starts_with("GET /users") {
        "HTTP/1.1 200 OK\r\n\r\nUser list page"
    } 
    else {
        "HTTP/1.1 200 OK\r\n\r\nWelcome to the server"
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

---

# Understanding `String::from_utf8_lossy`

When data is read from a TCP stream, it arrives as **raw bytes**, not text.

Example buffer contents:

```
[71, 69, 84, 32, 47, 104, 101, 108, 108, 111]
```

These bytes represent characters in UTF-8 encoding.

To inspect the request, the program converts the byte buffer into a string:

```
String::from_utf8_lossy(&buffer)
```

### Why "lossy"?

Some network data might not be valid UTF-8.
`from_utf8_lossy` ensures the program **does not crash** by replacing invalid bytes with a safe placeholder character.

Example:

```
invalid byte → �
```

This makes request parsing safer during development.

---

# Example Incoming Request

When a browser connects to the server, it sends a request like this:

```
GET /hello HTTP/1.1
Host: localhost:7878
User-Agent: Mozilla/5.0
Accept: */*
```

The server checks the first line:

```
GET /hello HTTP/1.1
```

and determines which response to send.

---

# Example Responses

### Request

```
http://localhost:7878/hello
```

Response:

```
Hello from Rust server
```

---

### Request

```
http://localhost:7878/users
```

Response:

```
User list page
```

---

### Request

```
http://localhost:7878/
```

Response:

```
Welcome to the server
```

---

# Learning Outcome

This update introduces an important backend concept:

**Routing**

The server now:

1. Reads the HTTP request
2. Converts raw bytes into readable text
3. Determines the requested route
4. Sends an appropriate response

This is the **foundation of all web frameworks**, where routing maps URLs to handler functions.

---

# Next Improvements

Future updates could include:

* Proper HTTP request parsing
* JSON API responses
* HTTP headers
* multithreading for concurrent connections
* modular router implementation
* structured request/response types
