use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_node(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("🔗 Node listening on port: {}", port);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            socket.read(&mut buffer).await.unwrap();
            println!("📩 Received: {:?}", buffer);
        });
    }
}
