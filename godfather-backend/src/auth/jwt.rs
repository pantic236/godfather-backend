use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Algorithm, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: String,      // user id
    pub username: String,
    pub role: String,
    pub exp: usize,       // expiration
    pub iat: usize,       // issued at
}

pub fn create_token(user_id: i64, username: &str, role: &str, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    
    let claims = JwtClaims {
        sub: user_id.to_string(),
        username: username.to_string(),
        role: role.to_string(),
        exp: (now + 3600) as usize, // 1 hour
        iat: now as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))?;

    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> Result<JwtClaims, Box<dyn std::error::Error>> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)?;
    
    Ok(token_data.claims)
}