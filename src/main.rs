use crate::hello::hello_service_server::HelloServiceServer;
use crate::hello::HelloServiceImpl;
use tonic::transport::Server;

mod hello;

const ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ADDR.parse()?;

    // Create the services
    let hello_service = HelloServiceServer::new(HelloServiceImpl);
    let (_, health_service) = tonic_health::server::health_reporter();

    // Start the server
    println!("twote-api running on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(hello_service)
        .serve(addr)
        .await?;

    Ok(())
}
