# Phase 2 – Concurrency with Thread Pool

## Overview

In Phase 1, the Rust server could handle HTTP requests but processed them **sequentially**.
This means the server handled one request at a time, causing other incoming requests to wait.

Phase 2 introduces **concurrency** by implementing a **Thread Pool**.
A thread pool allows multiple worker threads to process incoming requests simultaneously.

This significantly improves the server's ability to handle multiple clients.

---

# Problem with Phase 1

The Phase 1 architecture looked like this:

```
Client Request
      ↓
Main Thread
      ↓
handle_connection()
      ↓
Response Sent
```

Only one request could be processed at a time.

Example:

```
Request 1 → processed
Request 2 → waits
Request 3 → waits
```

---

# Solution: Thread Pool Architecture

With a thread pool, the server delegates work to worker threads.

```
Client Request
      ↓
Main Thread
      ↓
Thread Pool
      ↓
Worker Thread
      ↓
handle_connection()
```

Example execution:

```
Request 1 → Worker 0
Request 2 → Worker 1
Request 3 → Worker 2
Request 4 → Worker 3
```

Multiple requests are handled **simultaneously**.

---

# Thread Pool Design

The thread pool contains:

* A collection of **worker threads**
* A **job queue**
* A **message channel** used to distribute work

Structure:

```
ThreadPool
 ├── Workers (Vec<Worker>)
 └── Sender (Job Dispatcher)
```

Each worker continuously waits for incoming jobs.

---

# Thread Pool Implementation

File created:

```
src/thread_pool.rs
```

Core structure:

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
```

Where a **Job** is defined as:

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

This allows any function or closure to be executed by the worker threads.

---

# Worker Threads

Each worker thread runs in a loop waiting for new jobs:

```rust
loop {
    let job = receiver.lock().unwrap().recv().unwrap();
    job();
}
```

This allows threads to continuously process tasks without being recreated.

---

# Integrating Thread Pool with Server

The server now creates a thread pool in `main.rs`:

```rust
let pool = ThreadPool::new(4);
```

Incoming TCP connections are sent to the thread pool:

```rust
pool.execute(|| {
    handle_connection(stream);
});
```

Instead of being handled directly by the main thread.

---

# Expected Behavior

### Before Thread Pool

```
Client1 → handled
Client2 → waiting
Client3 → waiting
```

### After Thread Pool

```
Client1 → Worker 0
Client2 → Worker 1
Client3 → Worker 2
Client4 → Worker 3
```

Multiple requests are processed concurrently.

---

# Observing the Change

If logging is enabled:

```rust
println!("Worker {} got a job", id);
```

Terminal output will show:

```
Worker 0 got a job
Worker 1 got a job
Worker 2 got a job
```

This confirms that multiple workers are handling incoming requests.

---

# Warnings During Compilation

Rust may display warnings such as:

```
field `workers` is never read
field `thread` is never read
field `id` is never read
```

These occur because the thread pool stores worker metadata that will be used later for **graceful shutdown and thread management**.

The warnings are expected and do not affect functionality.

---

# Updated Project Structure

After Phase 2:

```
rust-http-server/
│
├── src/
│   ├── main.rs
│   └── thread_pool.rs
│
└── Cargo.toml
```

---

# Key Concepts Learned

This phase introduces several important systems programming concepts:

* Thread pools
* Worker thread architecture
* Message passing with channels
* Shared ownership with `Arc`
* Mutual exclusion with `Mutex`
* Concurrent request handling

These are foundational techniques used in modern backend systems.

---

# Next Phase

The next phase will introduce a **structured HTTP request parser**.

Instead of detecting routes using string matching:

```
request.starts_with("GET /hello")
```

We will parse the request into a structured object:

```rust
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>
}
```

This step will transform the server into a **mini HTTP framework**.
