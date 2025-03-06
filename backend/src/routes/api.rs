use axum:: {
    routing::post,
    routing::get,
    Router,
    Json,
    http::StatusCode,
};

use crate::routes::error::ApiError;
use crate::service::stock_service;
use crate::models::stocks::{Query, Stock};

use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use http::Method;

pub async fn task_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .route("/get-stock-data", post(get_stock_data))
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

async fn get_stock_data(Json(payload) : Json<Query>) -> Result<Json<Stock>, ApiError> {
    let response = stock_service::query_api(payload.ticker.as_str(), "2001-09").await?;
    Ok(Json(response))
}

