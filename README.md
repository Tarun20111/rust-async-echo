# Echo Program

A simple TCP client/server echo program implemented in Rust using Tokio for async networking.

## Overview

This project implements a concurrent TCP echo server and client:
- The **server** listens for incoming connections and echoes back any messages it receives
- The **client** connects to the server, sends a message, and prints the echoed response
- The server can handle multiple simultaneous client connections

## Requirements

- Rust 1.70+ (uses 2024 edition)
- Tokio async runtime

## Building

```bash
# Build both binaries
cargo build

# Or build individually
cargo build --bin echo-server
cargo build --bin echo-client
```

## Usage

### Running the Server

Start the echo server (listens on `127.0.0.1:9000`):

```bash
cargo run --bin echo-server
```

You should see output indicating the server is running and accepting connections.

### Running the Client

In a separate terminal, run the client with a message:

```bash
cargo run --bin echo-client "Hello, world!"
```

The client will:
1. Connect to the server
2. Send your message
3. Print the echoed response from the server

### Testing Multiple Clients

You can test concurrent connections by running multiple clients simultaneously:

```bash
# Terminal 1
cargo run --bin echo-client "First client message"

# Terminal 2 (at the same time)
cargo run --bin echo-client "Second client message"

# Terminal 3 (at the same time)  
cargo run --bin echo-client "Third client message"
```

Each client should receive its own message echoed back correctly.

## Example Session

```bash
# Terminal 1 - Start server
$ cargo run --bin echo-server
Server: got connection from :127.0.0.1:54321
Server: got connection from :127.0.0.1:54322

# Terminal 2 - Run client
$ cargo run --bin echo-client "Testing echo functionality"
got response:Testing echo functionality from server

# Terminal 3 - Run another client simultaneously
$ cargo run --bin echo-client "Another message"
got response:Another message from server
```

## Architecture

- **Async/Concurrent**: Uses Tokio for non-blocking I/O and spawns tasks for each client connection
- **Error Handling**: Comprehensive error handling with the `anyhow` crate
- **Simple Protocol**: Direct TCP message echoing (no framing protocol)

## Limitations

- Messages are limited by the 1024-byte buffer size
- No message framing protocol (assumes messages fit in single read/write operations)
- Server runs indefinitely (use Ctrl+C to stop)

## Dependencies

- `tokio` - Async runtime and networking
- `anyhow` - Error handling
