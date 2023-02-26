use anyhow::{Error, Result};
use protobuf::cache_server::{Cache, CacheServer};
use protobuf::{CacheGetRequest, CacheGetResponse, CacheSetRequest, CacheSetResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod protobuf {
    tonic::include_proto!("cache");
}

#[derive(Debug, Default)]
pub struct CacheService {}

#[tonic::async_trait]
impl Cache for CacheService {
    async fn get(
        &self,
        _request: Request<CacheGetRequest>,
    ) -> Result<Response<CacheGetResponse>, Status> {
        let response = CacheGetResponse {
            value: "Hello World!".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn set(
        &self,
        _request: Request<CacheSetRequest>,
    ) -> Result<Response<CacheSetResponse>, Status> {
        let response = CacheSetResponse { success: true };

        Ok(Response::new(response))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let addr = "[::1]:50051".parse()?;
    let cache_service = CacheService::default();

    Server::builder()
        .add_service(CacheServer::new(cache_service))
        .serve(addr)
        .await?;

    Ok(())
}
