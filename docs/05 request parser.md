# Phase 2 – HTTP Request Parser

## Overview

In Phase 1, the server handled HTTP requests by directly checking raw request strings such as:

```
GET /hello HTTP/1.1
```

This approach works for very simple routing but becomes difficult to maintain as the application grows.

In Phase 2, we introduce a **Request Parser** that converts the raw HTTP request into a structured Rust object. This allows the server to work with clearly defined fields like method, path, version, and headers instead of raw text.

---

## Why a Request Parser is Needed

When a browser or client sends a request, the server receives something like:

```
GET /users HTTP/1.1
Host: localhost:7878
User-Agent: Mozilla/5.0
Accept: */*
```

Without parsing, the server would need to manually inspect the string every time it needs information.

A parser converts this raw text into a structured format that is easier to use inside the program.

---

## Request Structure

The parsed request is stored inside a `Request` struct.

```rust
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}
```

### Fields

| Field   | Description                                  |
| ------- | -------------------------------------------- |
| method  | HTTP method such as GET or POST              |
| path    | Requested route such as `/hello` or `/users` |
| version | HTTP protocol version                        |
| headers | Key-value map containing HTTP headers        |

---

## Request Parsing Implementation

The parser converts the raw request string into a `Request` struct.

```rust
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String,String>,
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

        for line in lines {

            if line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }

        }

        Request {
            method,
            path,
            version,
            headers,
        }
    }
}
```

---

## How the Parser Works

### Step 1 – Split the Request into Lines

```
GET /users HTTP/1.1
Host: localhost:7878
User-Agent: Mozilla/5.0
```

The parser splits this into individual lines using:

```rust
request.lines()
```

---

### Step 2 – Extract the Request Line

The first line contains the core request information.

```
GET /users HTTP/1.1
```

This line is split into parts:

| Part     | Meaning          |
| -------- | ---------------- |
| GET      | HTTP Method      |
| /users   | Request Path     |
| HTTP/1.1 | Protocol Version |

---

### Step 3 – Parse Headers

Each remaining line contains headers.

Example:

```
Host: localhost:7878
User-Agent: Mozilla/5.0
```

The parser separates key and value using:

```
split_once(": ")
```

These values are stored in a `HashMap`.

---

## Integration with the Server

The parser is used inside the connection handler.

```rust
let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);

let request = Request::parse(&request_string);

println!("method: {}, path: {}", request.method, request.path);
```

Now routing logic can use structured fields instead of raw strings.

Example:

```rust
if request.path == "/hello" {
    // return hello response
}
```

---

## Benefits of This Approach

1. Cleaner routing logic
2. Easier debugging
3. Structured request handling
4. Foundation for POST and body parsing
5. Similar architecture used in real web frameworks

---

## Current Capabilities

The parser currently supports:

* Request line parsing
* HTTP header parsing

---

## Future Improvements

The next steps will extend the parser to support:

* Request body parsing
* POST requests
* Query parameters
* JSON payloads

---

## Project Architecture (Current)

```
Client
  ↓
TCP Listener
  ↓
Thread Pool
  ↓
Request Parser
  ↓
Route Handling
  ↓
HTTP Response
```

---

## Phase Status

| Phase   | Feature          | Status   |
| ------- | ---------------- | -------- |
| Phase 1 | TCP HTTP Server  | Complete |
| Phase 2 | Thread Pool      | Complete |
| Phase 2 | Request Parser   | Complete |
| Next    | Response Builder | Pending  |
