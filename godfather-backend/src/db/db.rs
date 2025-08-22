use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_db() -> anyhow::Result<SqlitePool> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://godfather.db".into());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
