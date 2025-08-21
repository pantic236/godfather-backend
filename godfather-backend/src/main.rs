mod routes;
mod handlers;
mod models;
mod state;
mod db;

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    // db setup
    let pool = db::init_db().await?;

    let state = state::AppState::new(pool);

    // routes
    let app: Router = routes::create_router(state);

    // serve
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    info!("backend running on http://{addr}!");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
