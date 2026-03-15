# Routes in the Rust HTTP Server

## What Is a Route?

A **route** defines how the server responds to a specific URL path.

Example:

```
/hello
/users
/about
```

Each route is mapped to a **handler function** that generates the response.

---

# Route Mapping

Routes are stored inside the router using a **HashMap**.

Example internal structure:

```
"/hello" → hello_handler
"/users" → users_handler
```

This allows the server to quickly locate the correct handler for a request.

---

# Registering Routes

Routes are registered during server startup.

Example:

```rust
router.get("/hello", hello_handler);
router.get("/users", users_handler);
```

This means:

```
GET /hello  → hello_handler
GET /users  → users_handler
```

---

# Route Lookup Process

When a request arrives:

```
GET /users HTTP/1.1
```

The router performs the following steps:

1. Extracts the path (`/users`)
2. Searches the route map
3. Finds the handler
4. Executes the handler

---

# Handling Unknown Routes

If no route matches the request path, the router returns a **404 response**.

Example:

```
GET /unknown
```

Response:

```
404 Not Found
```

---

# Advantages of Route Mapping

Using a route map provides several benefits:

* Fast lookups using HashMap
* Cleaner server logic
* Easy addition of new endpoints
* Better separation of responsibilities

---

# Example Route Table

| Path     | Handler       | Response Type    |
| -------- | ------------- | ---------------- |
| `/hello` | hello_handler | text/plain       |
| `/users` | users_handler | application/json |

---

# Summary

Routes define the **entry points of the web server**.
They allow the server to map URL paths to specific handler functions that generate responses.
