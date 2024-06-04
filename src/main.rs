mod rhm;
mod storage;
mod service;
mod client;

use service::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server().await?;
    Ok(())
}
