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
        .route("/balance", get(handlers::balance_handler::get_my_balance))
        .route("/users/:id/add_minutes", post(handlers::balance_handler::add_minutes))
        .with_state(app_state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}