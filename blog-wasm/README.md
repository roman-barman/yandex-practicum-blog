# blog-wasm

A modern, single-page application (SPA) frontend for the Yandex Practicum Blog, built with Yew and styled with Bootstrap.

## Features

- **Post Management**: View a list of posts, see post details, and manage posts (create, update, delete).
- **Authentication**: User registration and login functionality.
- **Pagination**: Efficiently browse through large numbers of posts.
- **Responsive Design**: Styled with Bootstrap for a clean look on all devices.
- **WebAssembly**: Compiled to WASM for high performance in the browser.

## Project Structure

- `src/components/`: Yew components (Home, Login, Register, PostsList, etc.).
- `src/route.rs`: Frontend routing definitions.
- `index.html`: The main entry point and template.

## Development

### Prerequisites

- [Trunk](https://trunkrs.dev/): A WASM web application bundler for Rust.

### Running Locally

To start a development server with hot-reloading:

```bash
trunk serve
```

The application will be available at `http://localhost:8080`.

### Building for Production

To build the project for production:

```bash
trunk build --release
```

The output will be in the `dist/` directory.

## Backend Connection

The frontend expects the backend server to be running at `http://localhost:3000`. Ensure that CORS is correctly configured on the server to allow requests from the frontend origin.
