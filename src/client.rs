use tonic::transport::Channel;
use rhm_service::rhm_service_client::RhmServiceClient;
use rhm_service::{GetRequest, SetRequest};

pub mod rhm_service {
    tonic::include_proto!("rhm_service");
}

#[derive(Debug)]
pub struct RhmClient {
    client: RhmServiceClient<Channel>,
}

impl RhmClient {
    pub async fn connect(addr: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let client = RhmServiceClient::connect(addr.into()).await?;
        Ok(RhmClient { client })
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(SetRequest {
            key: key.to_string(),
            value: value.to_string(),
        });

        self.client.set(request).await
            .map(|response| response.into_inner().result)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn get(&mut self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetRequest {
            key: key.to_string(),
        });

        self.client.get(request).await
            .map(|response| response.into_inner().value)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
