use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    models::{CreateUser, LoginReq, UpdateBalance, User},
    state::AppState,
};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    // user existence check
    let _ = sqlx::query!(
        "INSERT OR IGNORE INTO users(username, balance) VALUES(?, 60)",
        req.username
    )
    .execute(&*state.pool)
    .await
    .map_err(internal_err)?;

    // user fetch
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance 
         FROM users WHERE username = ?",
    )
    .bind(&req.username)
    .fetch_one(&*state.pool)
    .await
    .map_err(internal_err)?;

    Ok(Json(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, String> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, balance) VALUES (?, 60) 
         RETURNING id, username, balance",
    )
    .bind(payload.username)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(user))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, String> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance 
         FROM users"
    )
    .fetch_all(&*state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(users))
}

pub async fn update_balance(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBalance>,
) -> Result<Json<User>, String> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET balance = ?, minutes_balance = ? WHERE id = ? 
         RETURNING id, username, email, role, banned, created_at, last_login, balance, minutes_balance",
    )
    .bind(id)
    .fetch_one(&*state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(user))
}

fn internal_err<E: std::fmt::Display>(e: E) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}