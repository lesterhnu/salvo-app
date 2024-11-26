use salvo::prelude::*;
use salvo::oapi::extract::JsonBody;
use serde::Deserialize;

mod entity;
mod config;

#[tokio::main]
async fn main() {
    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    let server = Server::new(acceptor);
    let router = Router::new().get(goal);
    server.serve(router).await;

}
#[derive(Deserialize,Debug)]
struct User{
    name: String,
    age: i32,
}
#[handler]
async fn goal(req: JsonBody<User>)->String {
    let user = req.into_inner();
    println!("{:?}",user);
    user.name.clone()
}