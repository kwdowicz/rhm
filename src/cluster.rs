mod cluster_service;

use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Server")]
struct Opt {
    #[structopt(long, parse(try_from_str))]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    cluster_service::start_cluster(opt.listen).await?;
    Ok(())
}
