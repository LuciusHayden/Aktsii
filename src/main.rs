use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod models;
pub mod utils;

mod service;
mod routes;

#[tokio::main]
async fn main() {

    let app = routes::api::task_routes().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
 












