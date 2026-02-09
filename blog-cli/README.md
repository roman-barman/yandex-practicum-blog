# blog-cli

A command-line interface (CLI) tool for interacting with the Yandex Practicum Blog system. It allows users to manage their blog posts and accounts directly from the terminal.

## Features

- **Dual Protocol Support**: Communicates with the backend using either HTTP or gRPC.
- **Authentication**: Supports user registration and login.
- **CRUD Operations**: Create, Read, Update, and Delete blog posts.
- **Listings**: Fetch a list of posts with pagination.

## Usage

### General Options

```bash
cargo run -- [OPTIONS] <COMMAND>
```

Options:
- `-a, --address <ADDRESS>`: Server address (default: `http://localhost:3000` for HTTP, `http://localhost:50051` for gRPC).
- `-g, --grpc`: Use gRPC protocol instead of HTTP.

### Commands

- `register-user`: Register a new account.
- `login`: Log in to get an authentication token.
- `create-post`: Create a new blog post.
- `get-post`: Retrieve a specific post by ID.
- `get-posts-list`: List posts with optional limit and offset.
- `update-post`: Update an existing post.
- `delete-post`: Remove a post.

## Authentication

When you log in, the authentication token is stored locally in `~/.blog_token`. This token is automatically used for commands that require authorization (like creating or deleting posts).

## Example

```bash
# Register a user
cargo run -- register-user --username myuser --password mypass --email user@example.com

# Login
cargo run -- login --username myuser --password mypass

# Create a post
cargo run -- create-post --title "Hello World" --content "This is my first post"
```
