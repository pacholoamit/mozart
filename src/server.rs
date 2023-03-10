use crate::cache::{Cache, CacheKind, CacheLike};
use crate::prelude::*;
use crate::utils::ToShared;
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
    instance: ToShared<Box<dyn CacheLike>>,
}

impl Default for CacheService {
    fn default() -> Self {
        Self {
            instance: ToShared::new(Box::new(Cache::create(CacheKind::Default))),
        }
    }
}

#[tonic::async_trait]
impl CacheProto for CacheService {
    async fn get(&self, request: Request<CacheGetRequest>) -> GrpcResult<CacheGetResponse> {
        let req = request.into_inner();
        let cache = self.instance.lock();

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
