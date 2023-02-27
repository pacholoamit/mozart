use std::net::SocketAddr;
use std::sync::Arc;

use crate::cache::{Cache, CacheKind, CacheLike, HashMapCache};
use crate::prelude::*;
use protobuf::cache_server::{Cache as CacheProto, CacheServer};
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
    shared: Arc<Mutex<Box<dyn CacheLike>>>,
}

#[derive(Debug)]
pub struct SharedCache {
    cache: Arc<Mutex<Box<dyn CacheLike>>>,
}

impl Default for SharedCache {
    fn default() -> Self {
        Self {
            cache: Arc::new(Mutex::new(Box::new(Cache::create(CacheKind::Default)))),
        }
    }
}

#[tonic::async_trait]
impl CacheProto for CacheService {
    async fn get(&self, request: Request<CacheGetRequest>) -> GrpcResult<CacheGetResponse> {
        let req = request.into_inner();
        let cache = self.shared.lock().await;

        cache.get(&req.key);

        let value: String = match &cache.get(&req.key) {
            Some(value) => Value::String(value.to_string()).to_string(),
            None => String::from(""),
        };

        let response = CacheGetResponse { value };

        Ok(Response::new(response))
    }

    async fn set(&self, request: Request<CacheSetRequest>) -> GrpcResult<CacheSetResponse> {
        let req = request.into_inner();
        let mut cache = self.shared.lock().await;

        cache.set(&req.key, &Value::String(req.value));

        let response = CacheSetResponse { success: true };

        Ok(Response::new(response))
    }
}

pub async fn run(addr: SocketAddr) -> Result<()> {
    let cache_service = CacheService {
        shared: Arc::new(Mutex::new(Box::new(Cache::create(CacheKind::Default)))),
    };

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CacheServer::new(cache_service))
        .serve(addr)
        .await?;

    Ok(())
}
