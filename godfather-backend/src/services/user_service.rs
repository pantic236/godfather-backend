use axum::{Json, http::StatusCode};
use sqlx::SqlitePool;
use crate::{
    models::user::{User, RegisterReq, LoginReq, LoginResponse},
    auth::jwt::create_token,
};
use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn create_user(
    pool: &SqlitePool,
    req: RegisterReq,
) -> Result<Json<User>, (StatusCode, String)> {
    let hashed_password = hash(&req.password, DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password".to_string()))?;
    
    if sqlx::query("SELECT 1 FROM users WHERE username = ? OR email = ?")
        .bind(&req.username)
        .bind(&req.email)
        .fetch_optional(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?
        .is_some()
    {
        return Err((StatusCode::CONFLICT, "Username or email already exists".to_string()));
    }

    let role = req.role.unwrap_or("user".to_string());
    
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, role, password_hash, balance, minutes_balance) 
         VALUES (?, ?, ?, ?, 60, 60) 
         RETURNING id, username, email, role, banned, created_at, last_login, balance, minutes_balance, password_hash",
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&role)
    .bind(&hashed_password)
    .fetch_one(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create user: {}", e)))?;

    Ok(Json(user))
}

pub async fn authenticate_user(
    pool: &SqlitePool,
    jwt_secret: &str,
    req: LoginReq,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, email, role, banned, created_at, last_login, balance, minutes_balance, password_hash 
         FROM users WHERE username = ?",
    )
    .bind(&req.username)
    .fetch_one(pool)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let valid = verify(&req.password, &user.password_hash.as_ref().unwrap_or(&String::new()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Password verification failed".to_string()))?;
    
    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let _ = sqlx::query("UPDATE users SET last_login = datetime('now') WHERE id = ?")
        .bind(user.id)
        .execute(pool)
        .await;

    let token = create_token(user.id, &user.username, &user.role, jwt_secret)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token".to_string()))?;

    let response = LoginResponse {
        token,
        user,
    };

    Ok(Json(response))
}