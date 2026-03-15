# Phase 3 – Step 2: Converting `Response` Into a Valid HTTP Message

## Objective

In this step we implement the **core serialization function** of the response system:

```
Response::to_http_string()
```

This function converts the `Response` struct into a **valid HTTP/1.1 response message** that can be sent through the TCP stream.

Until now, the `Response` struct only stored data:

* Status code
* https://github.com/YASHIJARWAL/http_server_rust/tree/mainHeaders
* Body

This step transforms that structured data into the **actual HTTP protocol format** required by browsers, `curl`, and other clients.

---

# Why This Step Is Important

HTTP servers must send responses in a **very strict format**.

A valid HTTP response must follow this structure:

```
HTTP/1.1 <STATUS_CODE> <STATUS_TEXT>
Header-Key: value
Header-Key: value
Content-Length: <length>

<Response Body>
```

The empty line between headers and body is **mandatory**.
Without it, HTTP clients will fail to parse the response.

---

# Example HTTP Response

A valid response sent by our server should look like this:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 21

Hello from Rust server
```

Components:

| Part        | Meaning                     |
| ----------- | --------------------------- |
| Status Line | HTTP version + status code  |
| Headers     | Metadata about response     |
| Empty Line  | Separates headers from body |
| Body        | Actual response content     |

---

# Implementing `to_http_string()`

Add the following function inside the `impl Response` block in:

```
src/http/response.rs
```

```rust
pub fn to_http_string(&self) -> String {

    let mut response = format!("HTTP/1.1 {} OK\r\n", self.status_code);

    for (key, value) in &self.headers {
        response.push_str(&format!("{}: {}\r\n", key, value));
    }

    response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

    response.push_str("\r\n");

    response.push_str(&self.body);

    response
}
```

---

# How This Function Works

### 1. Create Status Line

```
HTTP/1.1 200 OK
```

```rust
let mut response = format!("HTTP/1.1 {} OK\r\n", self.status_code);
```

---

### 2. Add Headers

Each header is appended in the format:

```
Header-Key: value
```

```rust
for (key, value) in &self.headers {
    response.push_str(&format!("{}: {}\r\n", key, value));
}
```

---

### 3. Add Content-Length

HTTP requires the body length to be specified.

```rust
response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
```

---

### 4. Add Header–Body Separator

An empty line marks the end of headers.

```
\r\n
```

```rust
response.push_str("\r\n");
```

---

### 5. Append Body

Finally the response body is added.

```rust
response.push_str(&self.body);
```

---

# Resulting HTTP Message

The final string returned by `to_http_string()` looks like:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 21

Hello from Rust server
```

This string is then written directly to the TCP stream.

---

# Integration in `handle_connection()`

Once the response object is created, we convert it and send it to the client.

```rust
let response_string = response.to_http_string();

stream.write_all(response_string.as_bytes()).unwrap();
```

---

# Testing the Server

Run the server:

```
cargo run
```

Send a request using curl:

```
curl http://127.0.0.1:7878/hello
```

Expected output:

```
Hello from Rust server
```

Test JSON endpoint:

```
curl http://127.0.0.1:7878/users
```

Expected output:

```
[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]
```

---

# Debug Tip

For debugging, print the generated HTTP response:

```rust
println!("{}", response_string);
```

This helps verify that the server is sending a correctly formatted HTTP message.

---

# Phase 3 Progress

| Component          | Status    |
| ------------------ | --------- |
| Response Struct    | Completed |
| Header Builder     | Completed |
| Body Builder       | Completed |
| HTTP Serialization | Completed |

---

# Next Step

The next step will **integrate the Response system fully into the server routing logic** and remove all manual HTTP string formatting.

This prepares the architecture for the next major phase:

```
Phase 4 – Router Engine
```

Where endpoints will be registered using handler functions instead of manual `if` conditions.
