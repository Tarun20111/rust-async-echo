use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::time::{timeout, Duration};

async fn process_socket(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];
    loop {
        let read_len = match socket.read(&mut buf).await {
            Ok(len) => len,
            Err(err) => {
                println!("Read error: {err}");
                break;
            }
        };
        if read_len > 0 {
            match socket.write_all(&buf[0..read_len]).await {
                Ok(()) => (),
                Err(err) => {
                    println!("Write error: {err}, maybe client disconnected?");
                    break;
                }
            };
        } else {
            println!("Client disconnected");
            break;
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // ---
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Server listening on 127.0.0.1:9000 (Press Ctrl+C to stop)");

    loop {
        // ---
        tokio::select! {
            // ---
            // TODO: Using timeout to make select! responsive to Ctrl+C
            // In production, would use proper signal handling or async cancellation
            accept_result = timeout(Duration::from_millis(100), listener.accept()) => {
                // ---
                match accept_result {
                    Ok(Ok((socket, addr))) => {
                        println!("Server: got connection from :{addr}");
                        tokio::spawn(async move {
                            process_socket(socket).await;
                        });
                    }
                    Ok(Err(err)) => {
                        println!("Server: Error accepting connection: {err}");
                        break;
                    }
                    Err(_) => {
                        // Timeout - just continue to check for Ctrl+C
                        continue;
                    }
                }
            }
            _ = signal::ctrl_c() => {
                println!("\nServer: Received Ctrl+C, shutting down gracefully...");
                break;
            }
        }
    }

    println!("Server: shutdown complete");
    Ok(())
}
