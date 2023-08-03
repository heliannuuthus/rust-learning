use std::io;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn send(content: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:12345").await.unwrap();
    stream.write_all(content.as_bytes()).await?;
    Ok(())
}
