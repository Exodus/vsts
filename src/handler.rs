use axum::{
    extract::{Path, Query},
    headers::HeaderMap,
    http::Uri,
};
use std::collections::HashMap;
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
        } else if headers.contains_key("X-Forwarded-Uri") {
            return auth_with_x_forward_uri(headers).await;
        }
    }
    if let Some(query) = query {
        return auth_with_query(query).await;
    }
    Err(Error::MissingToken)
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

pub async fn auth_with_x_forward_uri(headers: HeaderMap) -> Result<String, Error> {
    let header = headers
    .get("X-Forwarded-Uri")
    .ok_or(Error::MissingToken)?
    .to_str()
    .map_err(|_| Error::InvalidToken)?;
    let uri = header.parse::<Uri>().map_err(|_| Error::InvalidToken)?;
    match uri.query_to_map().get("token") {
        Some(token) => validate_jwt(token),
        None => Err(Error::MissingToken)
    }
    
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

trait UriQueryToMap {
    fn query_to_map(&self) -> HashMap<&str, &str>;
}

impl UriQueryToMap for Uri {
    fn query_to_map(&self) -> HashMap<&str, &str> {
        let mut query_map = HashMap::new();
        if let Some(queries) = self.query() {
            for item in queries.split("&") {
                item.split_once("=").and_then(|(k,v)| query_map.insert(k,v));
            }
        }
        query_map
    }
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

    #[test]
    fn x_forward_uri_extract_token() {
        let uri: Uri = "http://github.com/Exodus/vsts/auth?key=value&foo=bar&token=123".parse().unwrap();
        let hm = HashMap::from([
            ("key", "value"),
            ("foo", "bar"),
            ("token", "123"),
            
        ]);
        assert_eq!(hm, uri.query_to_map());
    }
}