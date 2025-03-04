use axum:: {
    routing::post,
    Router,
};


pub async fn task_routes() -> Router {
    Router::new()
        .route("/", post(hello_world))
}

pub async fn hello_world() {
    println!("Hello world");
}














