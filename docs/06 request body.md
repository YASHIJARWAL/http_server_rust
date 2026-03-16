# Phase 2 – Final Step: HTTP Request Body Parsing

## Overview

In the final step of Phase 2, the HTTP request parser is extended to support **request body parsing**.

Earlier, the parser could only extract:

* HTTP Method
* Request Path
* HTTP Version
* Headers

However, real-world HTTP requests often include a **body**, especially for methods like **POST**, **PUT**, and **PATCH**.

This step completes the request parser by extracting the **body content** from the incoming HTTP request.

---

# Example HTTP Request

A client sending data to the server might send a request like:

```
POST /users HTTP/1.1
Host: localhost:7878
Content-Length: 17

{"name":"Alice"}
```

The final line represents the **request body**, which contains the data being sent to the server.

---

# Updated Request Structure

To support request bodies, the `Request` struct was updated.

```rust
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
```

### Field Description

| Field   | Description                     |
| ------- | ------------------------------- |
| method  | HTTP method such as GET or POST |
| path    | Requested endpoint              |
| version | HTTP protocol version           |
| headers | HashMap storing HTTP headers    |
| body    | Request payload sent by client  |

---

# Request Parsing Implementation

The parser extracts request components in three stages:

1. Parse request line
2. Parse headers
3. Parse request body

```rust
impl Request {

    pub fn parse(request: &str) -> Request {

        let mut lines = request.lines();

        let request_line = lines.next().unwrap();
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        let method = parts[0].to_string();
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        let mut headers = HashMap::new();

        // Parse headers
        for line in &mut lines {

            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        // Remaining lines form the request body
        let body = lines.collect::<Vec<&str>>().join("\n");

        Request {
            method,
            path,
            version,
            headers,
            body,
        }
    }
}
```

---

# Integration in the Server

Inside the connection handler, the request is parsed and printed for debugging.

```rust
let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);

let request = Request::parse(&request_string);

println!(
    "method:{} path:{} body:{}",
    request.method,
    request.path,
    request.body
);
```

This confirms that the parser correctly extracts request information.

---

# Testing the Parser

Start the server:

```
cargo run
```

Send a POST request using curl:

```
curl -X POST http://127.0.0.1:7878/users -d '{"name":"Alice"}'
```

Expected server output:

```
Connection received
Bytes read: 143
method:POST path:/users body:{"name":"Alice"}
```

---

# Phase 2 Achievements

At this stage, the server supports:

* Parsing HTTP request line
* Parsing HTTP headers
* Parsing HTTP request body
* Structured request representation
* Integration with routing logic

---

# Current Architecture

```
Client
  ↓
TCP Listener
  ↓
Thread Pool
  ↓
handle_connection()
  ↓
Request Parser
  ↓
Routing
  ↓
HTTP Response
```

---

# Phase Status

| Phase   | Component           | Status    |
| ------- | ------------------- | --------- |
| Phase 1 | TCP Server          | Completed |
| Phase 2 | Thread Pool         | Completed |
| Phase 2 | HTTP Request Parser | Completed |

---

# Next Phase

The next phase introduces a **Response abstraction layer**.

Instead of manually constructing responses using `format!()`, a `Response` struct will be created to manage:

* HTTP status codes
* Headers
* Response body

This will significantly improve code maintainability and bring the architecture closer to modern web frameworks.
