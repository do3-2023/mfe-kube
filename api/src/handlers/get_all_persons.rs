use super::ApiError;
use crate::{models::person::Person, ApiState};
use axum::{extract::State, Json};
use tracing::info;

pub async fn get_all_persons_handler(
    State(state): State<ApiState>,
) -> Result<Json<Vec<Person>>, ApiError> {
    let rows = {
        let pg_client = state.postgres_client.lock().await;

        pg_client
            .query(
                r#"
SELECT id, last_name, phone_number
FROM person;
            "#,
                &[],
            )
            .await
            .map_err(ApiError::DatabaseRequest)?
    };

    let persons = rows
        .iter()
        .map(|row| Person {
            id: row.get("id"),
            last_name: row.get("last_name"),
            phone_number: row.get("phone_number"),
        })
        .collect::<Vec<Person>>();

    info!("count: {} persons", persons.len());

    Ok(Json(persons))
}
