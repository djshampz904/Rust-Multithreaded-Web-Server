# 🚀 Rust Multithreaded Web Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📋 Overview

This project is a high-performance, multithreaded web server built with Rust. It leverages Rust's concurrency features and memory safety guarantees to create a robust and efficient HTTP server.

## ✨ Features

- 🧵 Thread pool implementation for handling concurrent connections
- 🔄 Graceful request handling and error management
- 💾 Static file serving capabilities
- ⚡ Fast response times with minimal resource usage
- 🛡️ Built with Rust's safety guarantees

## 🏗️ Architecture

The server is built using the following components:

- `main.rs`: Entry point that initializes the server and listener
- `lib.rs`: Core library functionality
- `thread_pool.rs`: Thread pool implementation for handling requests concurrently
- Worker threads that process incoming requests

## 🚦 Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-multithreaded-webserver.git
cd rust-multithreaded-webserver
```

2. Build the project:

```bash
cargo build --release
```

### Running the Server

Start the server with:

```bash
cargo run --release
```

The server will start listening on `127.0.0.1:7878` by default.

## 🧪 Testing

Run the test suite with:

```bash
cargo test
```

## 📊 Performance

The multithreaded design allows the server to handle numerous concurrent connections efficiently. The thread pool ensures that system resources are used optimally without creating a new thread for each request.

## 📝 Usage Examples

### Basic HTTP Request

You can send a request to the server using any HTTP client:

```bash
curl http://127.0.0.1:7878
```

### Static File Request

The server can serve static files from the `public` directory:

```bash
curl http://127.0.0.1:7878/hello.html
```

## 🔧 Configuration

Server configuration can be modified in the `main.rs` file:

- Port number
- Thread pool size
- Public directory path

## 🧠 Implementation Details

### Thread Pool

The thread pool implementation uses Rust's channels for communication between the main thread and worker threads:

- A fixed number of worker threads are created at startup
- Jobs are sent through a channel to available workers
- Each worker executes the job (handling a connection) in its own thread

### HTTP Parsing

The server implements basic HTTP/1.1 parsing to handle incoming requests, including:

- Request method detection
- Path and query parsing
- Header handling

### Response Generation

Responses are generated based on request paths:
- `/` - Serves the index page
- `/sleep` - Demonstrates the concurrency by delaying the response
- Other valid paths - Attempts to serve matching files
- Invalid paths - Returns a 404 response

## 📚 Learning Resources

This project is based on the web server tutorial from "The Rust Programming Language" book, with extended functionality and optimizations.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgements

- The Rust Programming Language Book
- The Rust community for their invaluable resources and support

## 📞 Contact

If you have any questions or suggestions, please open an issue in this repository.

---

⭐ Star this repository if you find it helpful!
