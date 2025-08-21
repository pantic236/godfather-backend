use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/login", post(handlers::login))
        .route("/users", post(handlers::create_user).get(handlers::list_users))
        .route("/users/:id/balance", put(handlers::update_balance))
        .with_state(state)
}
