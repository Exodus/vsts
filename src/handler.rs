use axum::{
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
};
use chrono::prelude::*;
use jsonwebtoken as jwt;

use super::error::Error;
use super::model;
use super::settings::CONFIG;

pub async fn create_jwt() -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(CONFIG.jwt.duration)
        .expect("valid timestamp")
        .timestamp();

    let claims = model::Claims {
        exp: expiration as usize,
    };

    let header = jwt::Header::new(jwt::Algorithm::HS512);
    jwt::encode(
        &header,
        &claims,
        &jwt::EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
    .map_err(|_| Error::JWTTokenCreationError)
}

pub async fn auth_with_path(
    axum::extract::Path(token): axum::extract::Path<String>,
) -> Result<String, Error> {
    println!("{}", token);
    let decoded = jwt::decode::<model::Claims>(
        &token,
        &jwt::DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &jwt::Validation::new(jwt::Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenError)?;

    Ok(decoded.claims.exp.to_string())
}

pub async fn auth_with_header(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Result<String, Error> {
    println!("{:?}", token);
    let token = token.0.token().to_string();
    println!("{}", token);
    validate_jwt(&token)
}

pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let decoded = jwt::decode::<model::Claims>(
        token,
        &jwt::DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &jwt::Validation::new(jwt::Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenError)?;

    Ok(decoded.claims.exp.to_string())
}

pub async fn healthz() -> String {
    "Healthy!".to_string()
}
