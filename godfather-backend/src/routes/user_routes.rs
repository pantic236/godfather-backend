use axum::{routing::{get, post, put}, Router};
use crate::{handlers, state::AppState};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/login", post(handlers::login))
        .route("/users", post(handlers::create_user).get(handlers::list_users))
        .route("/users/:id/balance", put(handlers::update_balance))
}