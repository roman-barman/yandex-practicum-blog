# blog-client

A shared library providing client implementations for the Yandex Practicum Blog system. It abstracts the communication logic for both HTTP and gRPC protocols.

## Features

- **Protocol Abstraction**: Unified interface for interacting with the blog backend via HTTP or gRPC.
- **Asynchronous**: Built on top of `reqwest` (HTTP) and `tonic` (gRPC) using `tokio`.
- **Command-based API**: Uses structured commands for all operations (Login, CreatePost, etc.).
- **Shared Domain Models**: Provides shared types for posts and users used by both CLI and potentially other consumers.

## Usage

This library is primarily used by `blog-cli`, but it can be integrated into any Rust project that needs to communicate with the blog server.

### Example

```rust
use blog_client::{Client, Protocol, LoginCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize client (defaults to HTTP)
    let mut client = Client::new(Protocol::Http("http://localhost:3000".to_string())).await?;

    // Perform login
    let token = client.login(LoginCommand::new("user", "password")).await?;
    println!("Token: {}", token);

    Ok(())
}
```

## Internal Modules

- `http_client`: Implementation of the HTTP/REST client.
- `grpc_client`: Implementation of the gRPC client.
- `proto`: Generated code from gRPC service definitions.
