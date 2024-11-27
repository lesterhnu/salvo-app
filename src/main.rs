use salvo::prelude::*;
use salvo::oapi::extract::JsonBody;
use serde::Deserialize;

mod entity;
mod config;
mod router;


#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    let server = Server::new(acceptor);
    let router = router::routers();
    let service:Service = router.into();
    // service.catcher(Ca)
    server.serve(service).await

}
