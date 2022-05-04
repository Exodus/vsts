use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JWT token not valid")]
    JWTTokenError,
    #[error("JWT token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("Invalid auth header")]
    InvalidAuthHeaderError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        (StatusCode::BAD_REQUEST, format!("Wrong Header Data: {}", e.name()))
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string())
        }
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR".to_string())
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
