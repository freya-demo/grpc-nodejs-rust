use grpc_nodejs_rust_demo::services::{
    counter::{SingleCounterServer, SingleCounterService},
    hello_world::{HelloWorldServer, HelloWorldService},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Start serving.");

    let addr = "[::1]:10000".parse().unwrap();

    let hello_world_service = HelloWorldService;
    let hello_world_server = HelloWorldServer::new(hello_world_service);

    let (single_counter_service, _jh) = SingleCounterService::spawn(None).await.unwrap();
    let single_counter_server = SingleCounterServer::new(single_counter_service);

    Server::builder()
        .add_service(hello_world_server)
        .add_service(single_counter_server)
        .serve_with_shutdown(addr, async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .unwrap();
}
