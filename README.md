rust-http-framework/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
│
├── src/
│   │
│   ├── main.rs
│   │
│   ├── server/
│   │   ├── mod.rs
│   │   ├── tcp_server.rs
│   │   ├── thread_pool.rs
│   │
│   ├── http/
│   │   ├── mod.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   ├── parser.rs
│   │
│   ├── router/
│   │   ├── mod.rs
│   │   ├── router.rs
│   │   ├── route.rs
│   │
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── logger.rs
│   │   ├── auth.rs
│   │
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── hello.rs
│   │   ├── users.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── json.rs
│   │   ├── config.rs
│   │
│   └── error/
│       ├── mod.rs
│       ├── http_error.rs
│
├── examples/
│   └── basic_server.rs
│
├── docs/
│   ├── architecture.md
│   ├── routing.md
│   ├── concurrency.md
│
└── tests/
    ├── router_tests.rs
    ├── http_tests.rs