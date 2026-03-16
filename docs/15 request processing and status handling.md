# Phase 6 --- Step 3 & Step 4

Middleware Safety and HTTP Status Handling

------------------------------------------------------------------------

# Step 3 --- Panic Safe Request Execution

## Objective

Ensure that a panic inside a handler does not crash the entire server.

In a multi-threaded HTTP server, if one request panics, the server
should return a **500 Internal Server Error** instead of terminating.

------------------------------------------------------------------------

## Approach

Rust provides:

    std::panic::catch_unwind()

This allows execution of code inside a **panic boundary**.

If a panic occurs, it can be caught and handled gracefully.

------------------------------------------------------------------------

## Implementation

Inside `handle_connection` the request processing logic is wrapped
inside `catch_unwind`.

``` rust
let response = logger::log_request(
    &request.method,
    &request.path,
    || {
        match std::panic::catch_unwind(|| process_request(&request, &router)) {
            Ok(resp) => resp,
            Err(_) => Response::new(500)
                .headers("content-type", "text/plain")
                .body("500 Internal Server Error"),
        }
    }
);
```

------------------------------------------------------------------------

## Flow After This Change

    Incoming TCP Connection
            │
    Request Parsing
            │
    Logger Middleware
            │
    catch_unwind Protection
            │
    process_request()
            │
    Router / Static Files
            │
    HTTP Response

------------------------------------------------------------------------

## Result

If a handler panics:

    HTTP/1.1 500 Internal Server Error
    Content-Type: text/plain

The **server continues running normally**.

------------------------------------------------------------------------

# Step 4 --- Correct HTTP Status Line

## Problem

Previously the response builder always produced:

    HTTP/1.1 <status_code> OK

Example:

    HTTP/1.1 404 OK

This is **not valid HTTP**.

Each status code must include the correct **reason phrase**.

------------------------------------------------------------------------

## Solution

A helper function was introduced to map status codes to their reason
phrase.

``` rust
fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "OK",
    }
}
```

------------------------------------------------------------------------

## Updated Response Builder

The `to_http_string()` function now constructs the status line properly.

``` rust
pub fn to_http_string(&self) -> String {

    let status_line = format!(
        "HTTP/1.1 {} {}\r\n",
        self.status_code,
        Self::status_text(self.status_code)
    );

    let mut response = status_line;

    for (key, value) in &self.headers {
        response.push_str(&format!("{}: {}\r\n", key, value));
    }

    response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
    response.push_str("\r\n");
    response.push_str(&self.body);

    response
}
```

------------------------------------------------------------------------

## Example Responses

Successful request

    HTTP/1.1 200 OK
    Content-Type: text/plain

Missing route

    HTTP/1.1 404 Not Found
    Content-Type: text/html

Server error

    HTTP/1.1 500 Internal Server Error

------------------------------------------------------------------------

# Outcome of Phase 6

By the end of Phase 6 the server now includes:

-   structured request pipeline
-   logging middleware
-   panic-safe request execution
-   custom 404 fallback
-   correct HTTP response formatting

The architecture is now **stable and production-style**.