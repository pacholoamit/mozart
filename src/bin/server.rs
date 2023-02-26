#![allow(unused)]

use anyhow::Result;
use mozart::cache::*;
use serde_json::json;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", 8080)).await?;

    println!("Started server on port 8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        socket.readable().await?;

        let mut buf = [0; 4096];
        match socket.read(&mut buf).await {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                let data = std::str::from_utf8(&buf[..n])?;
                println!("read {} bytes: {}", n, data);
                socket.write_all(data.as_bytes()).await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
