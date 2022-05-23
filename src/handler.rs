use axum::{
    extract::{Path, Query},
    headers::HeaderMap,
};
use chrono::prelude::*;
use jsonwebtoken as jwt;

use super::error::Error;
use super::model;
use super::settings::CONFIG;

/// Create and output a JWT Token
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
    .map_err(|_| Error::TokenCreation)
}

/// Authenticate via path
pub async fn auth_with_path(Path(token): Path<String>) -> Result<String, Error> {
    let decoded = jwt::decode::<model::Claims>(
        &token,
        &jwt::DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &jwt::Validation::new(jwt::Algorithm::HS512),
    )
    .map_err(|_| Error::InvalidToken)?;

    Ok(decoded.claims.exp.to_string())
}

pub async fn auth_with_header_or_query(
    headers: Option<HeaderMap>,
    query: Option<Query<model::Token>>,
) -> Result<String, Error> {
    if let Some(headers) = headers {
        if headers.contains_key("TOKEN") {
            return auth_with_header(headers).await;
        }
    }
    if let Some(query) = query {
        return auth_with_query(query).await;
    }
    return Err(Error::MissingToken);
}

/// Authenticate via Header
pub async fn auth_with_header(headers: HeaderMap) -> Result<String, Error> {
    let token = headers
        .get("TOKEN")
        .ok_or(Error::MissingToken)?
        .to_str()
        .map_err(|_| Error::InvalidToken)?;
    validate_jwt(token)
}

pub async fn auth_with_query(Query(token): Query<model::Token>) -> Result<String, Error> {
    validate_jwt(token.token.as_str())
}

/// Validate a JWT token
pub fn validate_jwt(token: &str) -> Result<String, Error> {
    let decoded = jwt::decode::<model::Claims>(
        token,
        &jwt::DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &jwt::Validation::new(jwt::Algorithm::HS512),
    )
    .map_err(|_| Error::InvalidToken)?;

    Ok(decoded.claims.exp.to_string())
}

///Health check
pub async fn healthz() -> String {
    "Healthy!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn token_created_valid() {
        let token = create_jwt().await.unwrap();
        assert!(
            !validate_jwt(&token).is_err(),
            "token creation/validation error"
        )
    }

    #[tokio::test]
    async fn create_token_success() {
        let token = create_jwt().await;
        assert!(!token.is_err(), "token creation error");
    }
}
