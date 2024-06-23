use hello_world::{
    hello_world_server::{HelloWorld, HelloWorldServer},
    DemoList,
};
use tonic::{transport::Server, Request, Response, Status};

mod hello_world {
    tonic::include_proto!("hello_world");
}

struct HelloWorldService;

#[tonic::async_trait]
impl HelloWorld for HelloWorldService {
    async fn hello_world(
        &self,
        request: Request<hello_world::HelloWorldRequest>,
    ) -> Result<Response<hello_world::HelloWorldResponse>, Status> {
        let req: &str = request.get_ref().hello_string.as_ref();
        let hello = if req.is_empty() { "hello" } else { req };
        let returned = format!("{} world!", hello);
        Ok(Response::new(hello_world::HelloWorldResponse {
            hello_world_string: returned,
        }))
    }

    async fn echo_list(&self, request: Request<DemoList>) -> Result<Response<DemoList>, Status> {
        let req = request.into_inner().demo_str;
        Ok(Response::new(DemoList { demo_str: req }))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Start serving.");

    let addr = "[::1]:10000".parse().unwrap();

    let hello_world_service = HelloWorldService;
    let hello_world_server = HelloWorldServer::new(hello_world_service);

    Server::builder()
        .add_service(hello_world_server)
        .serve(addr)
        .await
        .unwrap();
}
