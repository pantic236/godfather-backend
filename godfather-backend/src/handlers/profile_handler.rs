use axum::{
    Json, 
    extract::{Request},
    http::StatusCode,
};
use crate::{models::user::User, auth::jwt::JwtClaims};

#[derive(serde::Serialize)]
pub struct ProfileResponse {
    pub message: String,
    pub user_id: String,
    pub username: String,
    pub role: String,
}

pub async fn get_profile(
    request: Request,
) -> Result<Json<ProfileResponse>, StatusCode> {
    let claims = request.extensions().get::<JwtClaims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let response = ProfileResponse {
        message: "Profile information retrieved successfully".to_string(),
        user_id: claims.sub.clone(),
        username: claims.username.clone(),
        role: claims.role.clone(),
    };

    Ok(Json(response))
}