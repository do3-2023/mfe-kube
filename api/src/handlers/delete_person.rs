use super::ApiError;
use crate::{models::person::Person, ApiState};
use axum::{
    extract::{Path, State},
    Json,
};
use tracing::info;

pub async fn delete_person_handler(
    State(state): State<ApiState>,
    Path(id): Path<i32>,
) -> Result<Json<Person>, ApiError> {
    let row = {
        let pg_client = state.postgres_client.lock().await;

        pg_client
            .query_one(
                r#"
DELETE FROM person p
WHERE p.id = $1::INTEGER
RETURNING id, last_name, phone_number;
            "#,
                &[&id],
            )
            .await
            .map_err(ApiError::DatabaseRequest)?
    };

    let deleted_person = Person {
        id: row.get("id"),
        last_name: row.get("last_name"),
        phone_number: row.get("phone_number"),
    };

    info!("deleted person: {:?}", deleted_person);

    Ok(Json(deleted_person))
}
