use anyhow::{Error, Result};
use grpc_cache::cache_server::{Cache, CacheServer};
use grpc_cache::{CacheGetRequest, CacheGetResponse, CacheSetRequest, CacheSetResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod grpc_cache {
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
        Ok(Response::new(CacheGetResponse {
            value: "Hello World!".to_string(),
        }))
    }

    async fn set(
        &self,
        _request: Request<CacheSetRequest>,
    ) -> Result<Response<CacheSetResponse>, Status> {
        Ok(Response::new(CacheSetResponse { success: true }))
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
