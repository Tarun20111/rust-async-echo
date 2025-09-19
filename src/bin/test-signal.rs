use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Testing Ctrl+C handler...");

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl+C received!");
        }
    }

    println!("Exiting...");
    Ok(())
}
