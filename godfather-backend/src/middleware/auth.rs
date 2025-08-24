use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, HeaderMap},
};
use crate::auth::jwt::{verify_token, JwtClaims};

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&headers)?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let claims = verify_token(&token, &jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

pub async fn require_admin(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&headers)?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let claims = verify_token(&token, &jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    request.extensions_mut().insert(claims);
    
    Ok(next.run(request).await)
}

fn extract_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    auth_header
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)
}