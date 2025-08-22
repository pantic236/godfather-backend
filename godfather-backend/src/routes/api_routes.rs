use axum::Router;
use crate::{routes, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api", routes::user_routes::user_routes())  
        .with_state(state)
}