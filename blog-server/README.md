# blog-server

The backend API server for the Yandex Practicum Blog system. It provides both RESTful HTTP and gRPC interfaces for managing users and blog posts.

## Features

- **HTTP API**: Built with Actix-web.
- **gRPC API**: Built with Tonic.
- **Database**: PostgreSQL with SQLx for asynchronous database interactions.
- **Authentication**: JWT-based authentication.
- **Logging/Tracing**: Structured logging with Tracing.

## API Endpoints

### HTTP API

- `POST /api/auth/register`: Register a new user.
- `POST /api/auth/login`: Login and receive a JWT.
- `GET /api/posts`: Get a paginated list of posts.
- `GET /api/posts/{id}`: Get details of a specific post.
- `POST /api/posts`: Create a new post (Requires JWT).
- `PUT /api/posts/{id}`: Update an existing post (Requires JWT).
- `DELETE /api/posts/{id}`: Delete a post (Requires JWT).

### gRPC API

See `proto/blog.proto` for the service definition.

## Configuration

Configuration is managed via `config/` directory. You can set environment variables to override default settings (e.g., `APP_SERVER__HTTP_PORT=3000`).

## Running the Server

1. Run script:
   ```bash
   chmod +x ../scripts/init_db.sh
   ../scripts/init_db.sh
   ```
2. Start the server:
   ```bash
   APP_JWT__SECRET=<secret> cargo run
   ```
