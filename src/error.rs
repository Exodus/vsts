use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing TOKEN")]
    MissingToken,
    #[error("JWT token not valid")]
    InvalidToken,
    #[error("JWT token creation error")]
    TokenCreation,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::MissingToken => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            Error::InvalidToken => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Error::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
