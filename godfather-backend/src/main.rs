mod models;
mod handlers;
mod services;
mod auth;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in environment variables");
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:godfather.db".to_string());
    
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");
    
    let app_state = AppState {
        pool,
        jwt_secret,
    };
    
    let app = Router::new()
        .route("/health", get(handlers::user_handler::health))
        .route("/register", post(handlers::auth_handler::register))
        .route("/login", post(handlers::auth_handler::login))
        .route("/users", get(handlers::user_handler::get_users))
        .route("/users/:id/balance", get(handlers::balance_handler::get_balance))
        .route("/users/:id/add_bonus", post(handlers::balance_handler::add_bonus))
        .route("/sessions/start", post(handlers::session_handler::start_session))
        .route("/sessions/end", post(handlers::session_handler::end_session))
        .route("/sessions/:id", get(handlers::session_handler::get_session))
        .route("/machines/register", post(handlers::machine_handler::register_machine))
        .route("/machines/heartbeat", post(handlers::machine_handler::heartbeat))
        .route("/machines", get(handlers::machine_handler::get_machines))
        .route("/users/:id/ban", post(handlers::admin_handler::ban_user))
        .route("/users/:id/unban", post(handlers::admin_handler::unban_user))
        .route("/admin/users", get(handlers::admin_handler::get_all_users))
        .route("/admin/sessions", get(handlers::admin_handler::get_active_sessions))
        .with_state(app_state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}