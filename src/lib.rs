use chrono::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use warp::Rejection;
#[macro_use(lazy_static)]
extern crate lazy_static;

pub mod error;
pub mod settings;
use error::Error;

lazy_static! {
    pub static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

const JWT_SECRET: &[u8] = b"test";

type Result<T> = std::result::Result<T, Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    exp: usize,
}

pub fn create_jwt() -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(10))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}

pub async fn validate_jwt(jwt: String) -> WebResult<String> {
    let decoded = decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| warp::reject::custom(Error::JWTTokenError))?;

    Ok(decoded.claims.exp.to_string())
}
