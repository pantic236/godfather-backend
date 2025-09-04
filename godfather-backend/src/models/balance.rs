use serde::Serialize;

#[derive(Serialize)]
pub struct BalanceResponse {
    pub user_id: i64,
    pub total_minutes: i64,
    pub normal_minutes: i64,
    pub bonus_minutes: i64,
    pub message: String,
}