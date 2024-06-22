use super::ApiError;
use crate::ApiState;
use axum::extract::State;

pub async fn health_handler<'a>(State(state): State<ApiState>) -> Result<&'a str, ApiError> {
    // check DB health
    let pg_client = state.postgres_client.lock().await;
    pg_client
        .execute("SELECT 1;", &[])
        .await
        .map_err(ApiError::DatabaseConnection)?;

    Ok("healthy")
}
