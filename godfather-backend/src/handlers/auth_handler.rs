use axum::{Json, Extension};
use crate::{services::user_service, state::AppState};
use crate::models::user::{RegisterReq, LoginReq};
use axum::http::StatusCode;

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(req): Json<RegisterReq>,
) -> Result<Json<crate::models::user::User>, (StatusCode, String)> {
    user_service::create_user(&state.pool, req).await
}

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<crate::models::user::LoginResponse>, (StatusCode, String)> {
    user_service::authenticate_user(&state.pool, &state.jwt_secret, req).await
}