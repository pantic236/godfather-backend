use axum::{Json, extract::State};
use crate::{models::user::User, state::AppState};
use axum::http::StatusCode;

pub async fn health() -> &'static str {
    "ok"
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance, password_hash 
         FROM users"
    )
    .fetch_all(&state.pool)  
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch users: {}", e)))?;

    Ok(Json(users))
}