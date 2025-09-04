use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use crate::{
    models::session::{Session, StartSessionReq, EndSessionReq},
    state::AppState,
    models::machine::Machine,
};
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Serialize)]
pub struct SessionResponse {
    pub session: Session,
    pub message: String,
}

fn calculate_minutes_elapsed(started_at_str: &str) -> Result<i64, (StatusCode, String)> {
    let naive_dt = NaiveDateTime::parse_from_str(started_at_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse datetime: {}", e)))?;

    let started_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_dt, Utc);
    let now: DateTime<Utc> = Utc::now();

    let duration = now.signed_duration_since(started_at);
    let minutes_elapsed = (duration.num_seconds() / 60).max(1); // At least 1 minute

    Ok(minutes_elapsed as i64)
}

pub async fn start_session(
    State(state): State<AppState>,
    Json(req): Json<StartSessionReq>,
) -> Result<Json<SessionResponse>, (StatusCode, String)> {
    // balance check
    let user_minutes: i64 = sqlx::query_scalar!(
        "SELECT minutes_balance FROM users WHERE id = ?",
        req.user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    if user_minutes <= 0 {
        return Err((StatusCode::FORBIDDEN, "Insufficient balance".to_string()));
    }

    // machine check
    let machine: Machine = sqlx::query_as(
        "SELECT id, name, status FROM machines WHERE id = ? AND status = 'available'",
    )
    .bind(req.machine_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)))?
    .ok_or((StatusCode::NOT_FOUND, "Machine not found or not available".to_string()))?;

    let session: Session = sqlx::query_as(
        "INSERT INTO sessions (user_id, machine_id, started_at) 
         VALUES (?, ?, datetime('now')) 
         RETURNING id, user_id, machine_id, started_at, ended_at, minutes_consumed",
    )
    .bind(req.user_id)
    .bind(req.machine_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to start session: {}", e)))?;

    // machine used
    let _ = sqlx::query("UPDATE machines SET status = 'in_use' WHERE id = ?")
        .bind(req.machine_id)
        .execute(&state.pool)
        .await;

    let response = SessionResponse {
        session,
        message: format!("Session started successfully on machine {}", machine.name),
    };

    Ok(Json(response))
}

pub async fn end_session(
    State(state): State<AppState>,
    Json(req): Json<EndSessionReq>,
) -> Result<Json<SessionResponse>, (StatusCode, String)> {
    let session = sqlx::query_as::<_, Session>(
        "SELECT id, user_id, machine_id, started_at, ended_at, minutes_consumed FROM sessions WHERE id = ?",
    )
    .bind(req.session_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "Session not found".to_string()))?;

    if session.ended_at.is_some() {
        return Err((StatusCode::BAD_REQUEST, "Session already ended".to_string()));
    }

    // calculate elapsed time
    let started_at_naive = chrono::NaiveDateTime::parse_from_str(&session.started_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid timestamp format".to_string()))?;

    let started_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(started_at_naive, Utc);
    let now: DateTime<Utc> = Utc::now();

    let duration = now.signed_duration_since(started_at);
    let minutes_consumed = (duration.num_seconds() / 60).max(1) as i64; 

    // deduct from user balance
    let user_minutes: i64 = sqlx::query_scalar!(
        "SELECT minutes_balance FROM users WHERE id = ?",
        session.user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    if user_minutes < minutes_consumed {
        return Err((StatusCode::FORBIDDEN, "Insufficient balance to cover session time".to_string()));
    }

    let hours_consumed = ((minutes_consumed as f64) / 60.0).ceil() as i64;

    // update user's lifetime hours
    let _ = sqlx::query(
        "UPDATE users SET lifetime_hours = lifetime_hours + ? WHERE id = ?"
    )
    .bind(hours_consumed)
    .bind(session.user_id)
    .execute(&state.pool)
    .await;

    let updated_session = sqlx::query_as::<_, Session>(
        "UPDATE sessions SET ended_at = datetime('now'), minutes_consumed = ? WHERE id = ? 
         RETURNING id, user_id, machine_id, started_at, ended_at, minutes_consumed",
    )
    .bind(minutes_consumed)
    .bind(req.session_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to end session: {}", e)))?;

    let _ = sqlx::query(
        "UPDATE users SET minutes_balance = minutes_balance - ? WHERE id = ?",
    )
    .bind(minutes_consumed)
    .bind(session.user_id)
    .execute(&state.pool)
    .await;

    let _ = sqlx::query("UPDATE machines SET status = 'available' WHERE id = ?")
        .bind(session.machine_id)
        .execute(&state.pool)
        .await;

    let response = SessionResponse {
        session: updated_session,
        message: format!("Session ended successfully. {} minutes consumed. {} hours added to lifetime total.", 
                        minutes_consumed, hours_consumed),
    };

    Ok(Json(response))
}

pub async fn get_session(
    Path(session_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<SessionResponse>, (StatusCode, String)> {
    let session: Session = sqlx::query_as(
        "SELECT id, user_id, machine_id, started_at, ended_at, minutes_consumed FROM sessions WHERE id = ?",
    )
    .bind(session_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "Session not found".to_string()))?;

    let response = SessionResponse {
        session,
        message: "Session details retrieved successfully".to_string(),
    };

    Ok(Json(response))
}