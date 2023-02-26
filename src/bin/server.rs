#![allow(unused)]
use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};

pub mod cache {
    tonic::include_proto!("cache");
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Ok(())
}
