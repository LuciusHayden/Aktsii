use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod models;
pub mod utils;
pub mod routes;

mod service;

#[tokio::main]
async fn main() {

    let app = routes::api::task_routes().await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
 












