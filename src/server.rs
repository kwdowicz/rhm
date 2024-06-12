mod rhm;
mod storage;
mod server_service;
mod client;

use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Server")]
struct Opt {
    #[structopt(long, parse(try_from_str))]
    listen: SocketAddr,

    #[structopt(long, parse(try_from_str))]
    cluster: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    server_service::start_server(opt.listen, opt.cluster).await?;
    Ok(())
}
