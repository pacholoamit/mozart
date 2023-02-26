#![allow(unused)]

use anyhow::Result;
use mozart::{Cache, CacheTrait, HashMapCache};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<()> {
    let cache = Cache::create(Cache::HashMap);

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", 8080)).await?;

    Ok(())
}
