use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub banned: i64,
    pub created_at: String,
    pub last_login: Option<String>,
    pub balance: i64,
    pub minutes_balance: i64,
    pub bonus_minutes: i64,
    pub lifetime_hours: i64,
    pub password_hash: Option<String>,
}

impl User {
    pub fn is_banned(&self) -> bool {
        self.banned != 0
    }
    
    pub fn get_role(&self) -> &str {
        &self.role
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct RegisterReq {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateBalance {
    pub balance: i64,
    pub minutes_balance: i64,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}