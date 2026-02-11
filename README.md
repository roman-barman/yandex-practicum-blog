# Yandex Practicum Blog

A multi-component blog system written in Rust, featuring a backend server, a WebAssembly frontend, a CLI tool, and a client library.

## Project Structure

This repository is a Rust workspace containing the following projects:

- **[blog-server](./blog-server)**: The backend API server supporting both HTTP (Actix-web) and gRPC (Tonic).
- **[blog-wasm](./blog-wasm)**: A modern frontend built with Yew, compiled to WebAssembly.
- **[blog-cli](./blog-cli)**: A command-line interface tool to interact with the blog system.
- **[blog-client](./blog-client)**: A shared library providing HTTP and gRPC client implementations for the blog system.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Trunk](https://trunkrs.dev/) (for building the WASM frontend)
- [Docker](https://www.docker.com/) (for running the database)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (for database migrations)

## Getting Started

### 1. Database Setup

The server requires a PostgreSQL database. You can start one using Docker and the init script:

```bash
chmod +x scripts/init_db.sh
./scripts/init_db.sh
```

### 2. Running the Server

```bash
APP_JWT__SECRET=<secret> cargo run -p blog-server
```

The server will start an HTTP API on port 3000 and a gRPC server on port 50051 (defaults).

### 3. Running the WASM Frontend

```bash
cd blog-wasm
trunk serve
```

The app will be available at `http://localhost:8080`.

### 4. Using the CLI

```bash
cargo run -p blog-cli -- --help
```

## License

MIT or Apache-2.0