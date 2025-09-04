use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize}; 

use crate::{
    models::user::User,
    state::AppState,
    models::balance::BalanceResponse, 
};
#[derive(Deserialize)]
pub struct AddBonusReq {
    pub minutes: i64,
}

pub async fn add_bonus(
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
    Json(req): Json<AddBonusReq>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    if req.minutes <= 0 {
        return Err((StatusCode::BAD_REQUEST, "Minutes must be positive".to_string()));
    }
    
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET bonus_minutes = bonus_minutes + ? WHERE id = ? 
         RETURNING id, username, email, role, banned, created_at, last_login, balance, minutes_balance, bonus_minutes, password_hash",
    )
    .bind(req.minutes)
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;
    
    let response = BalanceResponse {
        user_id: user.id,
        total_minutes: user.minutes_balance + user.bonus_minutes,
        normal_minutes: user.minutes_balance,
        bonus_minutes: user.bonus_minutes,
        message: format!("Successfully added {} bonus minutes to user {}. Total balance: {} minutes", 
                        req.minutes, user.username, user.minutes_balance + user.bonus_minutes),
    };
    
    Ok(Json(response))
}

pub async fn get_balance(
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance, bonus_minutes, password_hash FROM users WHERE id = ?",
    )
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;
    
    let response = BalanceResponse {
        user_id: user.id,
        total_minutes: user.minutes_balance + user.bonus_minutes,
        normal_minutes: user.minutes_balance,
        bonus_minutes: user.bonus_minutes,
        message: "Balance retrieved successfully".to_string(),
    };
    
    Ok(Json(response))
}