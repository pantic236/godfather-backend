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
        .unwrap_or_else(|_| "515785423c0c730a5c1a2f40a8a2fd44".to_string());
    
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
        .with_state(app_state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("Server running on http://{}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}