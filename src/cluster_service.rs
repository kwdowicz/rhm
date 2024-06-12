use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use cluster_service::cluster_service_server::{ClusterService, ClusterServiceServer};
use cluster_service::{AddServerRequest, AddServerResponse};
use std::collections::HashSet;


pub mod cluster_service {
    tonic::include_proto!("cluster_service");
}

#[derive(Debug)]
pub struct ImplClusterService {
    servers: Arc<Mutex<HashSet<SocketAddr>>>,
}

#[tonic::async_trait]
impl ClusterService for ImplClusterService {
    async fn add_server(&self, request: Request<AddServerRequest>) -> Result<Response<AddServerResponse>, Status> {
        let req = request.into_inner();
        let mut servers = self.servers.lock().await;
        let mut result = String::from("Unknown");
        match req.addr.parse::<SocketAddr>() {
            Ok(socket) => { servers.insert(socket.clone()); result = format!("Added: {} to cluster", socket); },
            Err(e) => result = format!("Can't add socket to cluster: {e}"),
        }

        Ok(Response::new(AddServerResponse {
            result: result,
        }))
    }
}

pub async fn start_cluster(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let impl_cluster_service = ImplClusterService { servers: Arc::new(Mutex::new(HashSet::new())) };

    println!("Cluster listening on {}", addr);

    Server::builder()
        .add_service(ClusterServiceServer::new(impl_cluster_service))
        .serve(addr)
        .await?;

    Ok(())
}
