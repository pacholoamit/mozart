use anyhow::{Error, Result};
use grpc_cache::cache_client::CacheClient;
use grpc_cache::{CacheSetRequest};

pub mod grpc_cache {
    tonic::include_proto!("cache");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = CacheClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CacheSetRequest {
        key: "Hello".to_string(),
        value: "World!".to_string(),
    });

    let response = client.set(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
