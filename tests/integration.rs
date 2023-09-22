include!(concat!(env!("OUT_DIR"), "/hello.rs"));

use crate::hello_service_client::HelloServiceClient;

static ADDR: &str = "http://0.0.0.0:9090";

#[tokio::test]
async fn test_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect(ADDR).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    assert_eq!(response.into_inner().reply, "Hello, Tonic!");

    Ok(())
}

#[tokio::test]
async fn test_client_creation() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static(ADDR)
        .connect()
        .await
        .expect("Can't create a channel");

    let mut client = HelloServiceClient::new(channel);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    assert_eq!(response.into_inner().reply, "Hello, Tonic!");

    Ok(())
}
