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

    #[error("JWT token not valid - {0}")]
    InvalidToken(String),

    #[error("JWT token creation error")]
    TokenCreation,
}

impl Error {
    pub fn status(&self) -> StatusCode {
        match self {
            Error::MissingToken => StatusCode::UNAUTHORIZED,
            Error::InvalidToken(_) => StatusCode::NOT_FOUND,
            Error::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status(), self.to_string()).into_response()
    }
}
