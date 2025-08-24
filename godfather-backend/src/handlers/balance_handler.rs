use axum::{
    Json, 
    extract::{Path, State},
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use crate::{models::user::User, state::AppState, auth::jwt::verify_token};

#[derive(Deserialize)]
pub struct AddMinutesReq {
    pub minutes: i64,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub user_id: i64,
    pub minutes_balance: i64,
    pub message: String,
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

pub async fn add_minutes(
    headers: HeaderMap,  
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
    Json(req): Json<AddMinutesReq>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    let token = extract_token(&headers)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "No authorization token".to_string()))?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error".to_string()))?;
    
    let claims = verify_token(&token, &jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    
    if claims.role != "admin" {
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }
    
    if req.minutes <= 0 {
        return Err((StatusCode::BAD_REQUEST, "Minutes must be positive".to_string()));
    }
    
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET minutes_balance = minutes_balance + ? WHERE id = ? 
         RETURNING id, username, email, role, banned, created_at, last_login, balance, minutes_balance, password_hash",
    )
    .bind(req.minutes)
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, format!("User not found: {}", e)))?;
    
    let response = BalanceResponse {
        user_id: user.id,
        minutes_balance: user.minutes_balance,
        message: format!("Successfully added {} minutes to user {}. New balance: {} minutes", 
                        req.minutes, user.username, user.minutes_balance),
    };
    
    Ok(Json(response))
}

pub async fn get_balance(
    headers: HeaderMap,  
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    let token = extract_token(&headers)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "No authorization token".to_string()))?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error".to_string()))?;
    
    let claims = verify_token(&token, &jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    
    let requesting_user_id: i64 = claims.sub.parse()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid user ID in token".to_string()))?;
    
    if requesting_user_id != user_id && claims.role != "admin" {
        return Err((StatusCode::FORBIDDEN, "Can only check your own balance".to_string()));
    }
    
    let minutes_balance = sqlx::query_scalar!(
        "SELECT minutes_balance FROM users WHERE id = ?",
        user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;
    
    let response = BalanceResponse {
        user_id,
        minutes_balance,
        message: "Balance retrieved successfully".to_string(),
    };
    
    Ok(Json(response))
}

pub async fn get_my_balance(
    headers: HeaderMap,  
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    let token = extract_token(&headers)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "No authorization token".to_string()))?;
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Server configuration error".to_string()))?;
    
    let claims = verify_token(&token, &jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;
    
    // get user id
    let user_id: i64 = claims.sub.parse()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid user ID in token".to_string()))?;
    
    // get user balance
    let minutes_balance = sqlx::query_scalar!(
        "SELECT minutes_balance FROM users WHERE id = ?",
        user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;
    
    let response = BalanceResponse {
        user_id,
        minutes_balance,
        message: "Your balance retrieved successfully".to_string(),
    };
    
    Ok(Json(response))
}