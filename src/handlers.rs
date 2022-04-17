use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use warp::Rejection;

use super::error::Error;
use super::settings::CONFIG;
use super::models;

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

pub async fn validate_jwt(jwt: String) -> WebResult<String> {
    let decoded = decode::<models::Claims>(
        &jwt,
        &DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenError)?;

    Ok(decoded.claims.exp.to_string())
}
