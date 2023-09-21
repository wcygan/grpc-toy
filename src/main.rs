use crate::hello::hello_service_server::HelloServiceServer;
use crate::hello::HelloServiceImpl;
use tonic::transport::Server;
use tracing_subscriber::FmtSubscriber;

mod hello;

const ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let addr = ADDR.parse()?;

    // Create the services
    let hello_service = HelloServiceServer::new(HelloServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    println!("running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(hello_service)
        .serve(addr)
        .await?;

    Ok(())
}
