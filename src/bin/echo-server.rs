use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::io;

async fn process_socket(mut socket: TcpStream) {
    // ---
    let mut buf = vec![0; 1024];
    loop {
        let read_len = match socket.read(&mut buf).await {
            Ok(len) => len,
            Err(err) => {
                println!("Read error: {err}",);
                break;
            }
        };
        if read_len > 0 {
            match socket.write_all(&buf[0..read_len]).await {
                Ok(()) => (),
                Err(err) => {
                    println!("Write error: {err}, maybye client disconnected?");
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

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("Server: got connection from :{addr}");

        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}
