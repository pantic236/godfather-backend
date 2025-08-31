use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode, 
};
use serde::Serialize;
use crate::{
    models::machine::{Machine, RegisterMachineReq, HeartbeatReq},
    state::AppState,
};

#[derive(Serialize)]
pub struct MachineResponse {
    pub machine: Machine,
    pub message: String,
}

pub async fn register_machine(
    State(state): State<AppState>,
    Json(req): Json<RegisterMachineReq>,
) -> Result<Json<MachineResponse>, (StatusCode, String)> {
    let machine_name = req.name; 
    
    let existing = sqlx::query_scalar!(
        "SELECT id FROM machines WHERE name = ?",
        machine_name 
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;
    
    if existing.is_some() {
        return Err((StatusCode::CONFLICT, "Machine with this name already exists".to_string()));
    }
    
    let machine = sqlx::query_as::<_, Machine>(
        "INSERT INTO machines (name, status, last_seen_at) 
         VALUES (?, 'ONLINE', datetime('now')) 
         RETURNING id, name, status, last_seen_at",
    )
    .bind(&machine_name) 
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to register machine".to_string()))?;
    
    let response = MachineResponse {
        machine,
        message: format!("Machine {} registered successfully", machine_name),
    };
    
    Ok(Json(response))
}

pub async fn heartbeat(
    State(state): State<AppState>,
    Json(req): Json<HeartbeatReq>,
) -> Result<Json<MachineResponse>, (StatusCode, String)> {
    let machine = sqlx::query_as::<_, Machine>(
        "UPDATE machines SET status = 'ONLINE', last_seen_at = datetime('now') 
         WHERE id = ? 
         RETURNING id, name, status, last_seen_at",
    )
    .bind(req.machine_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "Machine not found".to_string()))?;
    
    let response = MachineResponse {
        machine,
        message: "Heartbeat received, machine status updated to ONLINE".to_string(),
    };
    
    Ok(Json(response))
}

pub async fn get_machines(
    State(state): State<AppState>,
) -> Result<Json<Vec<Machine>>, (StatusCode, String)> {
    let machines = sqlx::query_as::<_, Machine>(
        "SELECT id, name, status, last_seen_at FROM machines ORDER BY name"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch machines".to_string()))?;
    
    Ok(Json(machines))
}