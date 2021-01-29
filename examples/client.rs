use db_rs::repl::REPL;

/// Create and start an instance of KVDB Server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = match std::env::args().nth(1) {
        Some(addr) => addr,
        None => "http://127.0.0.1:50051".to_string(),
    };

    eprintln!("Client starting on {}", addr);
    REPL::start(addr).await?;

    Ok(())
}
