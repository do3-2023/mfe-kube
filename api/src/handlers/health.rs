use crate::errors::ApiError;
use axum::extract::State;
use sqlx::{postgres::any::AnyConnectionBackend, Pool, Postgres};

pub async fn health_handler(
    State(pool): State<Pool<Postgres>>,
) -> anyhow::Result<&'static str, ApiError> {
    // check DB health
    pool.acquire().await?.ping().await?;

    Ok("healthy")
}
