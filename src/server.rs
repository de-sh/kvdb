use bytes::Bytes;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tonic::{transport, Request, Response, Status};

use crate::{
    kvdb_proto::{
        kvdb_server::{Kvdb, KvdbServer},
        Byte, KeyValue, Null,
    },
    store::{ExecResult, Store},
};

pub struct Server {
    /// A cross thread sharable pointer to a common data-store
    store: Arc<Mutex<Store<Bytes, Bytes>>>,
}

impl Server {
    /// Create and initialize a server as service with gRPC interfaces.
    pub async fn start(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        transport::Server::builder()
            .add_service(KvdbServer::new(Self {
                store: Arc::new(Mutex::new(Store::new())),
            }))
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl Kvdb for Server {
    /// RPC that maps KEY to VALUE, if it doesn't already exist on Server.
    async fn set(&self, args: Request<KeyValue>) -> Result<Response<Null>, Status> {
        let args = args.into_inner();
        let (key, value) = (args.key, args.value);
        match self
            .store
            .lock()
            .await
            .set(Bytes::from(key), Bytes::from(value))
        {
            ExecResult::Failed => Err(Status::already_exists("Key in use")),
            ExecResult::Success => Ok(Response::new(Null {})),
        }
    }

    /// RPC that returns VALUE mapped to KEY, erring if it doesn't exist.
    async fn get(&self, args: Request<Byte>) -> Result<Response<Byte>, Status> {
        let key = args.into_inner().body;
        match self.store.lock().await.get(Bytes::from(key)) {
            Err(_) => Err(Status::not_found("Key not in use")),
            Ok(value) => Ok(Response::new(Byte {
                body: value.to_vec(),
            })),
        }
    }

    /// RPC that removes a KEY -> VALUE mapping, erring if it doesn't exist.
    async fn del(&self, args: Request<Byte>) -> Result<Response<Null>, Status> {
        let key = args.into_inner().body;
        match self.store.lock().await.del(Bytes::from(key)) {
            ExecResult::Failed => Err(Status::not_found("Key not in use")),
            ExecResult::Success => Ok(Response::new(Null {})),
        }
    }
}
