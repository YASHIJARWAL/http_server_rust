# Phase 4 – Router Integration

## Objective

Phase 4 introduces a **routing system** to the HTTP server.
Instead of manually checking request paths using `if-else` statements, the server now uses a **Router module** that maps URL paths to handler functions.

This change makes the server **scalable, maintainable, and closer to real-world backend frameworks**.

---

# Why Routing Is Needed

Before Phase 4, request handling looked like this:

```
if request.path == "/hello" {
    ...
}
else if request.path == "/users" {
    ...
}
else {
    ...
}
```

Problems with this approach:

* Difficult to maintain when routes increase
* Code becomes cluttered
* Not modular

To solve this, we introduce a **Router abstraction**.

---

# Router-Based Architecture

After Phase 4 the flow becomes:

```
Incoming Request
      │
Request Parser
      │
Router
      │
Route Lookup
      │
Handler Function
      │
Response
```

This architecture separates **network logic** from **application logic**.

---

# Project Structure After Phase 4

```
src
│
├── main.rs
├── thread_pool.rs
├── router.rs
│
└── http
     ├── mod.rs
     ├── request.rs
     └── response.rs
```

---

# Router Responsibilities

The router is responsible for:

* Registering routes
* Mapping paths to handler functions
* Returning the correct response for a request
* Handling unknown routes

---

# Route Registration

Routes are registered during server startup.

Example:

```rust
router.get("/hello", hello_handler);
router.get("/users", users_handler);
```

Each route connects a **URL path** to a **handler function**.

---

# Router Execution

When a request arrives:

1. The request is parsed.
2. The router checks the path.
3. The router finds the corresponding handler.
4. The handler generates a response.

Example:

```
GET /hello
```

Router finds:

```
"/hello" → hello_handler
```

The handler generates the response.

---

# Thread Safety

Since the server uses a **thread pool**, the router must be shared safely between threads.

This is achieved using:

```
Arc<Router>
```

`Arc` allows multiple threads to access the same router instance safely.

---

# Benefits of Phase 4

* Clean request handling
* Modular architecture
* Easy route addition
* Real backend design pattern

---

# Example Endpoints

| Route    | Description                     |
| -------- | ------------------------------- |
| `/hello` | Returns a greeting message      |
| `/users` | Returns a sample JSON user list |

---

# Summary

Phase 4 transforms the server from a simple request handler into a **structured backend system** with a routing layer similar to modern web frameworks.
