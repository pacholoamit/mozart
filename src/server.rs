use std::net::SocketAddr;
use std::sync::Arc;

use crate::cache::{Cache, CacheKind, CacheLike, HashMapCache};
use crate::prelude::*;
use protobuf::cache_server::{Cache as CacheProtoLike, CacheServer};
use protobuf::{CacheGetRequest, CacheGetResponse, CacheSetRequest, CacheSetResponse};
use serde_json::Value;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

pub mod protobuf {
    tonic::include_proto!("cache");
}

type GrpcResult<T> = Result<Response<T>, Status>;

#[derive(Debug)]
pub struct CacheService {
    cache: HashMapCache,
}

#[tonic::async_trait]
impl CacheProtoLike for CacheService {
    async fn get(&self, request: Request<CacheGetRequest>) -> GrpcResult<CacheGetResponse> {
        let req = request.into_inner();

        let mut cache = self.cache.clone();

        let value: String = match cache.get(&req.key) {
            Some(value) => Value::String(value.to_string()).to_string(),
            None => String::from(""),
        };

        let response = CacheGetResponse { value };

        Ok(Response::new(response))
    }

    async fn set(&self, request: Request<CacheSetRequest>) -> GrpcResult<CacheSetResponse> {
        let req = request.into_inner();

        let mut cache = self.cache.clone();

        cache.set(&req.key, &Value::String(req.value));

        let response = CacheSetResponse { success: true };

        Ok(Response::new(response))
    }
}

pub async fn run(addr: SocketAddr) -> Result<()> {
    let cache_service = CacheService {
        cache: HashMapCache::default(),
    };

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CacheServer::new(cache_service))
        .serve(addr)
        .await?;

    Ok(())
}
