# mini-curl

A lightweight curl like HTTP client built in Rust.

This project is a simple command line tool that makes HTTP requests and prints the response status, headers and body. It supports basic methods like GET and POST and can print JSON responses.

---

## Features

- Make HTTP requests from the command line
- Supports *GET, POST, PUT* (via `-X`)
- Send request bodies using `-d`
- Async and non blocking using Tokio

---

## Project Structure
```

mini_curl/
├── Cargo.toml
└── src/
    ├── main.rs   # entry point
    ├── cli.rs    # Command line argument parsing
    └── http.rs   # HTTP request and response handling

```

---

## Usage

- Simple GET req
```
cargo run -- https://httpbin.org/get
```

- POST req with data
```
cargo run -- -X POST -d '{"name":"rust"}' https://httpbin.org/post
```

- PUT req
```
cargo run -- -X PUT https://httpbin.org/put
```
