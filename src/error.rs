use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    JWTTokenError,
    JWTTokenCreationError,
    XForwardedUriError,
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
            Error::XForwardedUriError => (StatusCode::BAD_REQUEST, "Malformed URI in X-FORWARDED-Uri Header".to_string()),
            Error::JWTTokenError => (StatusCode::FORBIDDEN, "JWT Token not valid".to_string()),
            Error::JWTTokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "JWT token creation error".to_string()),
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
