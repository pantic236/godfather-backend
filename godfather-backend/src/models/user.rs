use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub balance: i64,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UpdateBalance {
    pub balance: i64,
}

#[derive(Deserialize)]
pub struct LoginReq {
    pub username: String,
}
