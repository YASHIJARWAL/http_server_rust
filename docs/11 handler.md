# Handler Functions

## What Is a Handler?

A **handler** is a function responsible for generating the HTTP response for a specific route.

Each route in the router points to a handler function.

Example:

```
/hello → hello_handler
```

---

# Handler Function Signature

All handlers follow a common structure:

```rust
fn handler(request: &Request) -> Response
```

### Parameters

| Parameter | Description             |
| --------- | ----------------------- |
| `request` | The parsed HTTP request |

### Return Value

| Value      | Description                               |
| ---------- | ----------------------------------------- |
| `Response` | The HTTP response sent back to the client |

---

# Example Handler

```rust
fn hello_handler(_req: &Request) -> Response {

    Response::new(200)
        .headers("Content-Type","text/plain")
        .body("Hello from Rust Server")

}
```

This handler returns a **simple text response**.

---

# JSON Handler Example

```rust
fn users_handler(_req: &Request) -> Response {

    Response::new(200)
        .headers("Content-Type","application/json")
        .body(r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#)

}
```

This handler returns a **JSON response**.

---

# Handler Execution Flow

When a request arrives:

```
GET /hello
```

The router performs:

```
/hello → hello_handler
```

The handler generates the response which is sent back to the client.

---

# Advantages of Handlers

Handlers provide:

* Clear separation of business logic
* Reusable endpoint functions
* Easy API expansion
* Cleaner server architecture

---

# Handler in Request Lifecycle

```
Client Request
      │
TCP Server
      │
Request Parser
      │
Router
      │
Handler Function
      │
Response
      │
Client
```

---

# Summary

Handlers contain the **actual business logic of the server**.
Each route triggers a handler that processes the request and returns the appropriate response.
