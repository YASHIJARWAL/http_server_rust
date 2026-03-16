# Rust HTTP Server

A minimal HTTP server built from scratch in Rust that demonstrates core backend and systems programming concepts such as networking, concurrency, routing, middleware, and static file serving.

This project implements the fundamental components that power modern web frameworks and servers.

---

# Project Motivation

Most backend projects use frameworks that hide the internal workings of a web server.

This project focuses on building the core HTTP server architecture manually in order to understand:

* TCP networking
* HTTP protocol structure
* request parsing
* response construction
* routing systems
* middleware execution
* concurrency using thread pools
* static file serving
* binary-safe responses

---

# Features

## Networking

* TCP server using `TcpListener`
* Concurrent request handling using a custom thread pool

## HTTP Implementation

* HTTP request parsing
* HTTP response construction
* HTTP status codes
* header handling
* Content-Length support

## Routing System

Custom router implementation that maps paths to handlers.

```rust
router.get("/hello", hello_handler);
router.get("/users", user_handler);
```

---

## Middleware Support

A middleware layer processes requests before responses are sent.

Example logging middleware:

```
METHOD PATH
GET /hello
GET /users
```

---

## Static File Server

Files are served from the `public` directory.

Example URLs:

```
/index.html
/style.css
/script.js
```

---

## Binary File Support

The server supports binary responses using `Vec<u8>` instead of `String`.

This allows serving:

* HTML
* CSS
* JavaScript
* JSON
* PNG images
* JPEG images
* GIF files
* Icons

---

## Custom Error Handling

The server supports:

```
404 Not Found
500 Internal Server Error
```

Custom pages can be served such as:

```
/error404.html
```

---

# Project Architecture

```
src
│
├── main.rs
├── thread_pool.rs
│
├── http
│   ├── request.rs
│   └── response.rs
│
├── routes
│   ├── router.rs
│   └── handler.rs
│
├── middleware
│   └── logger.rs
│
├── static_files
│   └── file_server.rs
```

Static assets:

```
public/
 ├── index.html
 ├── style.css
 ├── script.js
 └── error404.html
```

Documentation:

```
docs/
 ├── 01 server explanation.md
 ├── 02 routing.md
 ├── 03 Content headers.md
 ├── 04 Concurrency.md
 ├── 05 request parser.md
 ├── 06 request body.md
 ├── 07 response module creation.md
 ├── 08 http_string creation.md
 ├── 09 router integration.md
 ├── 10 route.md
 ├── 11 handler.md
 ├── 12 static files.md
 ├── 13 Middleware logger.md
 ├── 14 error 404 handling.md
 ├── 15 request processing and status handling.md
 └── 16 binary file support.md
---

# How to Run

## 1. Clone the repository

```
git clone <repo-url>
cd rust-http-server
```

## 2. Run the server

```
cargo run
```

The server starts at:

```
http://127.0.0.1:7878
```

---

# Example Routes

```
GET /hello
GET /users
```

Example static files:

```
http://127.0.0.1:7878/index.html
http://127.0.0.1:7878/style.css
```

---

# Learning Outcomes

This project demonstrates knowledge of:

* Rust ownership model
* TCP networking
* HTTP protocol fundamentals
* concurrency with thread pools
* modular system design
* middleware architecture
* binary-safe file handling

---

# Future Improvements

Potential improvements for the project:

* HTTP request body parsing
* POST/PUT support
* JSON response helpers
* middleware chaining
* request logging improvements
* configuration system
* async support

---

# License

MIT License
