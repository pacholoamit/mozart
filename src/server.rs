use crate::cache::{Cache, CacheKind, CacheLike};
use crate::prelude::*;
use crate::utils::{serde_to_prost, Shared};
use protobuf::cache_server::{Cache as CacheProto, CacheServer};
use protobuf::{CacheGetRequest, CacheGetResponse, CacheSetRequest, CacheSetResponse};
use serde_json::Value;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod protobuf {
    tonic::include_proto!("cache");
}

type GrpcResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct CacheService {
    instance: Shared<Box<dyn CacheLike>>,
}

impl Default for CacheService {
    fn default() -> Self {
        Self {
            instance: Shared::new(Box::new(Cache::create(CacheKind::Default))),
        }
    }
}

#[tonic::async_trait]
impl CacheProto for CacheService {
    async fn get(&self, request: Request<CacheGetRequest>) -> GrpcResult<CacheGetResponse> {
        let req = request.into_inner();
        let cache = self.instance.lock();

        cache.get(&req.key);

        let value = &cache.get(&req.key);

        let value: prost_types::Value = match value {
            Some(value) => serde_to_prost(value),
            None => prost_types::Value { kind: None },
        };

        // TODO: Make this better
        let response = CacheGetResponse { value: Some(value) };

        Ok(Response::new(response))
    }

    async fn set(&self, request: Request<CacheSetRequest>) -> GrpcResult<CacheSetResponse> {
        let req = request.into_inner();
        let mut cache = self.instance.lock();

        cache.set(&req.key, &Value::String(req.value));

        let response = CacheSetResponse { success: true };

        Ok(Response::new(response))
    }
}

pub async fn run(addr: SocketAddr) -> Result<()> {
    let cache_service = CacheService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CacheServer::new(cache_service))
        .serve(addr)
        .await?;

    Ok(())
}
