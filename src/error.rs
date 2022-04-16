use serde::Serialize;
use std::convert::Infallible;
use warp::{http::StatusCode, reply, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    JWTTokenError,
    JWTTokenCreationError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("Not Found", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::JWTTokenError => Ok(reply::with_status(
                "JWT Token not valid",
                StatusCode::FORBIDDEN,
            )),
            Error::JWTTokenCreationError => Ok(reply::with_status(
                "JWT token creation error",
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
