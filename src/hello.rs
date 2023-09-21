include!(concat!(env!("OUT_DIR"), "/hello.rs"));

use crate::hello::hello_service_server::HelloService;
use tonic::{Request, Response, Status};

pub struct HelloServiceImpl;

#[tonic::async_trait]
impl HelloService for HelloServiceImpl {
    #[tracing::instrument(name = "say_hello", skip(self))]
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::info!("Processing say_hello request");

        let name = request.into_inner().name;

        let reply = HelloReply {
            reply: format!("Hello, {}!", name),
        };

        Ok(Response::new(reply))
    }
}
