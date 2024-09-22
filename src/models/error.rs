use serde::Serialize;
use std::error::Error;
use std::fmt;
use axum::http::StatusCode;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}


#[derive(Debug)]
pub struct NotFoundError;

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokémon not found")
    }
}

impl Error for NotFoundError {}

#[derive(Debug)]
pub struct ApiError(pub StatusCode);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API returned error: {}", self.0)
    }
}

impl Error for ApiError {}
