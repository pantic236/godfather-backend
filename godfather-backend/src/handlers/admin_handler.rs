use axum::{
    Json,
    extract::{Path, State}, 
    http::StatusCode,
};
use serde::Serialize; 
use crate::{
    models::user::User,
    state::AppState,
    models::session, 
};

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User, 
    pub message: String,
}

#[derive(Serialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    pub message: String,
}

#[derive(Serialize)]
pub struct SessionsResponse {
    pub sessions: Vec<session::Session>,
    pub message: String,
}

pub async fn ban_user(
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET banned = 1 WHERE id = ? 
         RETURNING id, username, email, role, banned, created_at, last_login, balance, minutes_balance, bonus_minutes, password_hash",
    )
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let username = user.username.clone(); 
    let response = UserResponse {
        user, 
        message: format!("User {} has been banned", username), 
    };

    Ok(Json(response))
}

pub async fn unban_user(
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET banned = 0 WHERE id = ? 
         RETURNING id, username, email, role, banned, created_at, last_login, balance, minutes_balance, bonus_minutes, password_hash",
    )
    .bind(user_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let username = user.username.clone();
    let response = UserResponse {
        user, 
        message: format!("User {} has been unbanned", username), 
    };

    Ok(Json(response))
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<UsersResponse>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance, bonus_minutes, password_hash FROM users ORDER BY username"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch users: {}", e)))?; 

    let response = UsersResponse {
        users,
        message: "Users retrieved successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn get_active_sessions(
    State(state): State<AppState>,
) -> Result<Json<SessionsResponse>, (StatusCode, String)> {
    let sessions = sqlx::query_as::<_, session::Session>( 
        "SELECT id, user_id, machine_id, started_at, ended_at, minutes_consumed FROM sessions WHERE ended_at IS NULL ORDER BY started_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch active sessions: {}", e)))?;

    let response = SessionsResponse {
        sessions,
        message: "Active sessions retrieved successfully".to_string(),
    };

    Ok(Json(response))
}

pub async fn get_top_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance, bonus_minutes, lifetime_hours, password_hash 
         FROM users 
         ORDER BY lifetime_hours DESC 
         LIMIT 10"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch top users".to_string()))?;

    Ok(Json(users))
}