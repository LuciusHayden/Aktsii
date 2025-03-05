use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    RequestError,
    AlphaVantageError,
}

impl fmt::Display for ApiError { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            ApiError::RequestError => write!(f, "Request Failed"),
            ApiError::AlphaVantageError => write!(f, "Error with AlphaVantage API (check api key and uses)"),
        }
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response { 
        let status = match self { 
            ApiError::RequestError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AlphaVantageError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
