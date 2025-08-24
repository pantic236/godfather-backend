use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Machine {
    pub id: i64,
    pub name: String,
    pub status: String,
}

