use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use crate::rhm::{Rhm, RhmResult};
use rhm_service::rhm_service_server::{RhmService, RhmServiceServer};
use rhm_service::{GetRequest, GetResponse, SetRequest, SetResponse};
use tonic::transport::Channel;
use cluster_service::cluster_service_client::ClusterServiceClient;
use cluster_service::AddServerRequest;

pub mod rhm_service {
    tonic::include_proto!("rhm_service");
}

pub mod cluster_service {
    tonic::include_proto!("cluster_service");
}

#[derive(Debug)]
pub struct MyRhmService {
    rhm: Arc<Mutex<Rhm>>,
}

#[tonic::async_trait]
impl RhmService for MyRhmService {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let req = request.into_inner();
        let mut rhm = self.rhm.lock().await;
        let result = rhm.set(&req.key, &req.value).await.map_err(|e| Status::internal(format!("Failed to set value: {}", e)))?;

        Ok(Response::new(SetResponse {
            result: result.value(),
        }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let rhm = self.rhm.lock().await;
        let result = rhm.get(&req.key);

        Ok(Response::new(GetResponse {
            value: result.value(),
            found: matches!(result, RhmResult::Value(_)),
        }))
    }
}

pub async fn start_server(addr: SocketAddr, cluster: Option<impl Into<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let rhm = Rhm::new().await?;
    let my_rhm_service = MyRhmService { rhm: Arc::new(Mutex::new(rhm)) };

    println!("Server listening on {}", addr);

    match cluster {
        Some(cluster_listen_addr) => {
            let mut client = ClusterServiceClient::connect(cluster_listen_addr.into()).await?;
            let request = tonic::Request::new(AddServerRequest {
                addr: addr.to_string()
            });

            client.add_server(request).await
                .map(|response| response.into_inner().result)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
        },
        None => (),
    }

    Server::builder()
        .add_service(RhmServiceServer::new(my_rhm_service))
        .serve(addr)
        .await?;

    Ok(())
}
