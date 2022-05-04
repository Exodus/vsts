use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use super::error::Error;
use super::settings::CONFIG;
use super::models;

const BEARER: &str = "Bearer ";

type Result<T> = std::result::Result<T, Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

pub fn create_jwt() -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(10))
        .expect("valid timestamp")
        .timestamp();

    let claims = models::Claims {
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
    .map_err(|_| Error::JWTTokenCreationError)
}

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
    .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<models::Claims>(
                &jwt,
                &DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;

            Ok(decoded.claims.exp.to_string())
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

pub async fn validate_jwt(jwt: String) -> WebResult<String> {
    println!("{}", jwt);
    let decoded = decode::<models::Claims>(
        &jwt,
        &DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenError)?;

    Ok(decoded.claims.exp.to_string())
}
