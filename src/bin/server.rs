#![allow(unused)]

use anyhow::Result;
use mozart::{Cache, CacheTrait};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut cache = Cache::create(Cache::HashMap);

    cache.set("wombo", &json!("combo"));

    println!("{:?}", cache.get("wombo"));

    // let listener = TcpListener::bind(&format!("127.0.0.1:{}", 8080)).await?;

    Ok(())
}
