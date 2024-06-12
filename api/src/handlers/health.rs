use super::ApiError;
use axum::extract::State;
use sqlx::{postgres::any::AnyConnectionBackend, Pool, Postgres};

#[tracing::instrument]
pub async fn health_handler<'a>(State(pool): State<Pool<Postgres>>) -> Result<&'a str, ApiError> {
    // check DB health
    pool.acquire()
        .await
        .map_err(ApiError::DatabaseConnection)?
        .ping()
        .await
        .map_err(ApiError::DatabaseRequest)?;

    Ok("healthy")
}
