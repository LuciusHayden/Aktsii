use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;
use serde_json::json;

use serde_json::Error as Serde_Error;
use reqwest::Error as Reqwest_Error;

#[derive(Debug)]
pub enum ApiError {
    RequestError,
    AlphaVantageError,
    Other(String),
}

impl fmt::Display for ApiError { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            ApiError::RequestError => write!(f, "Request Failed"),
            ApiError::AlphaVantageError => write!(f, "Error with AlphaVantage API (check api key and uses)"),
            ApiError::Other(msg) => write!(f, "Other error occured: {}", msg),
        }
    }
}

impl From<Serde_Error> for ApiError {
    fn from(value: Serde_Error) -> Self {
        ApiError::Other(format!("Issue with serde_json: {}", value).to_string())
    }
}

impl From<Reqwest_Error> for ApiError {
    fn from(_value: Reqwest_Error) -> Self {
        ApiError::RequestError
    }
}

impl std::error::Error for ApiError {
    fn description(&self) -> &str {
        "Error related to the API"
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response { 
        let (status, message)= match self { 
            ApiError::RequestError => (StatusCode::INTERNAL_SERVER_ERROR, "RequestError".to_string()),
            ApiError::AlphaVantageError => (StatusCode::INTERNAL_SERVER_ERROR, "Error with AlphaVantage".to_string()),
            ApiError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = json!({ "error": message });

        (status, axum::Json(body)).into_response()
    }
}
