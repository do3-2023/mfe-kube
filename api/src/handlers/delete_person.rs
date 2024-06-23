use super::ApiError;
use crate::ApiState;
use axum::extract::{Path, State};
use mysql::{params, prelude::Queryable};
use tracing::info;

pub async fn delete_person_handler(
    State(state): State<ApiState>,
    Path(id): Path<i32>,
) -> Result<(), ApiError> {
    {
        let mut conn = state
            .pool
            .get_conn()
            .map_err(ApiError::DatabaseConnection)?;

        conn.exec_drop(
            r#"
DELETE FROM person p
WHERE p.id = :id;
            "#,
            params! { "id" => id },
        )
        .map_err(ApiError::DatabaseRequest)?
    };

    info!("deleted person id: {:?}", id);

    Ok(())
}
