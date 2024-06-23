use super::ApiError;
use crate::ApiState;
use axum::extract::State;
use mysql::prelude::Queryable;

pub async fn health_handler<'a>(State(state): State<ApiState>) -> Result<&'a str, ApiError> {
    // check DB health
    {
        let mut conn = state
            .pool
            .get_conn()
            .map_err(ApiError::DatabaseConnection)?;

        conn.exec_drop("SELECT 1;", ())
            .map_err(ApiError::DatabaseConnection)?;
    }

    Ok("healthy")
}
