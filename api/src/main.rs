use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), mfe_api::StartError> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("APP_PORT").unwrap_or("3000".into());
    let addr = format!("{}:{}", host, port);
    info!("listening on http://{}/", addr);

    let database_url = std::env::var("DATABASE_URL").expect("no DATABASE_URL given");
    let pool = init_db(&database_url).await?;

    mfe_api::run_server(addr, pool).await
}

async fn init_db(database_url: &str) -> Result<Pool<Postgres>, mfe_api::StartError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(1))
        .connect(database_url)
        .await
        .map_err(mfe_api::StartError::CreatePostgresPool)?;

    // Database migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(mfe_api::StartError::PostgresMigration)?;

    Ok(pool)
}
