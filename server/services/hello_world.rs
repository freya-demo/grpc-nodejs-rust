use crate::compiled_protos::hello_world::{
    hello_world_server::HelloWorld, DemoList, HelloWorldRequest, HelloWorldResponse,
};
use tonic::{Request, Response, Status};

pub use crate::compiled_protos::hello_world::hello_world_server::HelloWorldServer;
pub struct HelloWorldService;

#[tonic::async_trait]
impl HelloWorld for HelloWorldService {
    async fn hello_world(
        &self,
        request: Request<HelloWorldRequest>,
    ) -> Result<Response<HelloWorldResponse>, Status> {
        let req: &str = request.get_ref().hello_string.as_ref();
        let hello = if req.is_empty() { "hello" } else { req };
        let returned = format!("{} world!", hello);
        Ok(Response::new(HelloWorldResponse {
            hello_world_string: returned,
        }))
    }

    async fn echo_list(&self, request: Request<DemoList>) -> Result<Response<DemoList>, Status> {
        let req = request.into_inner().demo_str;
        Ok(Response::new(DemoList { demo_str: req }))
    }
}
