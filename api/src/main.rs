use anyhow::{Context, Ok};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::{filter::Targets, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    // Tracing & logging
    tracing_subscriber::registry()
        .with(
            Targets::new()
                .with_target("sqlx", Level::DEBUG)
                .with_target("tower_http", Level::TRACE)
                .with_target("mfe_api", Level::TRACE),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let app_port = std::env::var("APP_PORT").unwrap_or("80".to_string());
    let database_url = std::env::var("DATABASE_URL").context("no DATABASE_URL given")?;

    let pool = init_db(database_url).await?;

    mfe_api::run_server(app_port, pool).await
}

async fn init_db(database_url: String) -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(2))
        .connect(&database_url)
        .await
        .context("could not connect to database")?;

    // Database migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("could not execute the database migrations")?;

    Ok(pool)
}
