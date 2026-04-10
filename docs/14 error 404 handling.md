# Phase 6 – Step 2

# Custom 404 Error Page Handling

## Overview

In this step we improve the server’s error handling by introducing a **custom 404 error page**.

Previously, when a requested route or file was not found, the server returned a simple text response such as:

```
404 Not Found
```

While functional, this approach does not provide a good user experience when accessed through a browser. In modern web servers, missing resources typically return a **custom HTML page** explaining that the requested resource could not be found.

This step implements a **fallback mechanism** that serves a dedicated `404.html` page from the `public` directory whenever a route and static file are both unavailable.

---

# Objective

The goals of this step are:

* Improve **error handling** for invalid routes
* Serve a **custom HTML error page**
* Extend the request pipeline with a **fallback mechanism**
* Make the server behave more like a **real web server**

---

# Custom Error Page

A new file is created inside the `public` directory.

Project structure:

```
public
│
├── index.html
├── style.css
├── script.js
└── error404.html
```

This HTML file is displayed when the requested resource does not exist.

Example `error404.html`:

```html
<!DOCTYPE html>
<html>
<head>
<title>404 Not Found</title>
</head>
<body>

<h1>404</h1>
<p>The requested page could not be found.</p>

</body>
</html>
```

---

# Request Handling Logic

The server processes incoming requests in the following order:

```
Incoming Request
       │
       ▼
Router
       │
Route found?
  /        \
Yes        No
 │          │
Handler   Static File Check
             │
        File exists?
        /        \
      Yes        No
       │          │
Serve File   Serve 404 Page
```

This ensures that every request results in a meaningful response.

---

# Implementation

The request handling logic inside `handle_connection` was updated to include a fallback mechanism.

```rust
let response = logger::log_request(
    &request.method,
    &request.path,
    || {

        let mut response = router.handle(&request);

        if response.status_code == 404 {

            if let Some(file_response) = serve_file(&request.path) {
                response = file_response;
            }
            else {

                if let Some(not_found_page) = serve_file("/error404.html") {
                    response = not_found_page;
                }
            }
        }

        response
    }
);
```

---

# Important Rust Concept

During implementation, it is important that the closure passed to the middleware returns a **Response object**.

Rust closures return the **last expression** in the block.

Correct example:

```
|| {
    let mut response = router.handle(&request);

    response
}
```

If the response is not returned, the closure returns `()` (unit type), which causes compilation errors when attempting to call methods like `to_http_string()`.

---

# Expected Behaviour

The server now responds differently depending on the request type.

### Valid Route

```
GET /hello
```

Response:

```
Hello from Rust Server
```

---

### Static File

```
GET /index.html
```

Response:

```
public/index.html
```

---

### Invalid Path

```
GET /unknown
```

Response:

```
public/error404.html
```

The browser displays the custom error page.

---

# Terminal Output Example

Example request logs:

```
Connection received
method:GET path:/hello body:
[LOG] GET /hello completed in 900µs

Connection received
method:GET path:/random body:
[LOG] GET /random completed in 1.8ms
```

---

# Benefits of Custom Error Pages

Implementing a custom 404 page provides several advantages:

* Improved **user experience**
* Clearer feedback for missing resources
* Professional server behaviour
* Consistent error handling

---

# Result of Step 2

After completing this step, the server now supports:

* Custom 404 error pages
* Static fallback handling
* Improved request pipeline structure

This enhancement moves the server closer to the behaviour of production-grade web servers.

---http://127.0.0.1:7878/hello

# Next Step

In **Phase 6 – Step 3**, we will further improve the architecture by introducing **centralized error handling and cleaner request pipelines**.
