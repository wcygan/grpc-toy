include!(concat!(env!("OUT_DIR"), "/hello.rs"));

use crate::hello_service_client::HelloServiceClient;

#[tokio::test]
async fn test_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect(format!("http://0.0.0.0:8080")).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    assert_eq!(response.into_inner().reply, "Hello, Tonic!");

    Ok(())
}
