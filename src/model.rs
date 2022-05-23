use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}
