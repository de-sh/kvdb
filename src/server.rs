use tokio::net::TcpListener;

/// An experimental serverlet
pub async fn serve(addr: String) {
    let listener = TcpListener::bind(addr.clone()).await.unwrap();
    loop {
        eprintln!("Server initialized: {}", addr);
        let (sock, _) = listener.accept().await.unwrap();
    }
}
