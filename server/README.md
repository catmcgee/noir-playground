# Noir Playground Server

Noir Playground allows developers to learn Noir through a set of interactive
challenges in a GUI. 

This is the server app.

## Prerequisites

1. [Rust and Cargo](https://www.rust-lang.org/tools/install)
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
2. [Noir and Nargo](https://noir-lang.org/getting_started/nargo_installation)
```curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash``` then ```noirup```

## Installing and Running

Run with cargo (recommended)

```bash
cargo update && cargo run
```

Alternatively, run with [Docker](https://www.docker.com/)

```bash
docker build -t server .
docker run -p 8080:8080 server
```

Your server will be running on `localhost:8080`

## Run the Client App

To have a working Playground, you will also need to run the client app.
Learn how to do that in the project's root [README](https://github.com/catmcgee/noir-playground#readme)

