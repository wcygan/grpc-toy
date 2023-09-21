include!(concat!(env!("OUT_DIR"), "/hello.rs"));

use crate::hello::hello_service_server::HelloService;
use tonic::{Request, Response, Status};

pub struct HelloServiceImpl;

#[tonic::async_trait]
impl HelloService for HelloServiceImpl {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let greeting = request.into_inner().greeting;

        let reply = HelloReply {
            reply: format!("Hello, {}!", greeting),
        };

        Ok(Response::new(reply))
    }
}
