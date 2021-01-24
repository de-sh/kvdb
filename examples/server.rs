use db_rs::server::Server;

/// Create and start an instance of KVDB Server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = match std::env::args().nth(1) {
        Some(addr) => addr,
        None => "127.0.0.1:50051".to_string(),
    }
    .parse()
    .unwrap();

    eprintln!("Server starting on {}", addr);
    Server::start(addr).await?;

    Ok(())
}
