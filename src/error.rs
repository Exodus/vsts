use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing TOKEN Header")]
    JWTTokenHeaderError,
    #[error("JWT token not valid")]
    JWTTokenError,
    #[error("JWT token creation error")]
    JWTTokenCreationError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::JWTTokenHeaderError => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Error::JWTTokenError => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::JWTTokenCreationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
