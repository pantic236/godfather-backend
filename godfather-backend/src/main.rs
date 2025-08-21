use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    pool: Arc<SqlitePool>,
}

#[derive(Serialize, FromRow)]
struct User {
    id: i64,
    username: String,
    balance: i64,
}

#[derive(Deserialize)]
struct LoginReq {
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    //logs
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
    .init();

    let database_url = 
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://godfather.db".into());

    //connect pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    //run migrations on startup
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState {
        pool: Arc::new(pool),
    };

    //routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/login", post(login))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    info!("backend running on http://{addr}!");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    // user existence check
    let _ = sqlx::query!(
        "INSERT OR IGNORE INTO users(username, balance) VALUES(?, 60)",
        req.username
    )
    .execute(&*state.pool)
    .await
    .map_err(internal_err)?;

    // user fetch
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, balance FROM users WHERE username = ?",
    )
    .bind(&req.username)
    .fetch_one(&*state.pool)
    .await
    .map_err(internal_err)?;

    Ok(Json(user))
}


fn internal_err<E: std::fmt::Display>(e: E) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
