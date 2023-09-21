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
        let name = request.into_inner().name;

        let reply = HelloReply {
            reply: format!("Hello, {}!", name),
        };

        Ok(Response::new(reply))
    }
}
