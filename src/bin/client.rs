use anyhow::{Error, Result};
use protobuf::cache_client::CacheClient;
use protobuf::{CacheGetRequest, CacheSetRequest};
pub mod protobuf {
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

    let request = tonic::Request::new(CacheGetRequest {
        key: "Hello".to_string(),
    });

    let response = client.get(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
