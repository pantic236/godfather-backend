use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub user_id: i64,
    pub machine_id: i64,
    pub started_at: String, 
    pub ended_at: Option<String>,
    pub minutes_consumed: i64,
}
#[derive(Deserialize)]
pub struct StartSessionReq {
    pub user_id: i64,
    pub machine_id: i64,
}

#[derive(Deserialize)]
pub struct EndSessionReq {
    pub session_id: i64,
}