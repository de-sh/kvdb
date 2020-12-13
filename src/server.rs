use tokio::net::TcpListener;

/// An experimental serverlet
pub async fn serve(addr: String) {
    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (sock, _) = listener.accept().await.unwrap();
    }
}