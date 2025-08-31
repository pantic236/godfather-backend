use serde::Serialize;
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Machine {
    pub id: i64,
    pub name: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct RegisterMachineReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct HeartbeatReq {
    pub machine_id: i64,
}