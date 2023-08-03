use std::io;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
mod client;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    for i in 0..4 {
        tokio::spawn(async move {
            client::send(format!("123456789: {}", i).as_str())
                .await
                .unwrap();
        });
    }
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let job_handler = tokio::spawn(async move {
            process_stream(&mut stream).await.unwrap();
        });
    }
}

async fn process_stream(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer: [u8; 64] = [0; 64];
    let mut content: Vec<u8> = Vec::new();
    while let Ok(r) = stream.read(&mut buffer).await {
        content.extend_from_slice(&buffer[..r]);
        if r <= 0 {
            break;
        }
    }
    println!("{:?}", String::from_utf8_lossy(&content));
    Ok(())
}
