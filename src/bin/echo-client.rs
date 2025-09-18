use anyhow::{anyhow, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    // ---

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <message>", args[0]));
    }

    // Connect to a peer
    let mut stream = match TcpStream::connect("127.0.0.1:9000").await {
        Ok(stream) => stream,
        Err(err) => {
            return Err(anyhow!("got error in connect:{err}"));
        }
    };
    let message = &args[1];

    match stream.write_all(message.as_bytes()).await {
        Ok(()) => {}
        Err(err) => {
            return Err(anyhow!("write error: {err}, sending to socket"));
        }
    }

    stream.shutdown().await?;

    let mut buf = vec![0; 1024];

    // TODO: this may truncate the reply. Should use [len][message],
    // That is server should send binary 4 byte length in big-endian
    // followed by the message. Then convert with u32:from_be_bytes
    //
    let read_len = match stream.read(&mut buf).await {
        // ---
        Ok(len) => len,
        Err(err) => {
            return Err(anyhow!("read error: {err}"));
        }
    };
    if read_len > 0 {
        // ---
        let msg = match std::str::from_utf8(&buf[0..read_len]) {
            Ok(msg) => msg,
            Err(err) => {
                return Err(anyhow!("Read back garbled none UTF msg from server:{err}"));
            }
        };
        println!("got response:{msg} from server");
        Ok(())
    } else {
        Err(anyhow!("Got zero bytes on read back from server"))
    }
}
