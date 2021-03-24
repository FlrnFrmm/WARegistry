mod proto;

use tonic::{transport::Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:10815".parse().unwrap();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(proto::say_server::SayServer::new(proto::waregistry::MySay::default()))
        .add_service(proto::push_server::PushServer::new(proto::waregistry::PushService::default()))
        .serve(addr)
        .await?;
    Ok(())
}