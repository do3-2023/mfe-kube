use super::ApiError;
use crate::{models::person::Person, ApiState};
use axum::{extract::State, Json};
use mysql::prelude::Queryable;
use tracing::info;

pub async fn get_all_persons_handler(
    State(state): State<ApiState>,
) -> Result<Json<Vec<Person>>, ApiError> {
    let persons = {
        let mut conn = state
            .pool
            .get_conn()
            .map_err(ApiError::DatabaseConnection)?;

        conn.query_map(
            "SELECT id, last_name, phone_number FROM person;",
            |(id, last_name, phone_number)| Person {
                id,
                last_name,
                phone_number,
            },
        )
        .map_err(ApiError::DatabaseRequest)?
    };

    info!("count: {} persons", persons.len());

    Ok(Json(persons))
}
