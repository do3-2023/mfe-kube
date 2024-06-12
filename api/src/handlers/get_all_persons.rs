use super::ApiError;
use crate::models::person::Person;
use axum::{extract::State, Json};
use sqlx::{Pool, Postgres};
use tracing::info;

#[tracing::instrument]
pub async fn get_all_persons_handler(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Person>>, ApiError> {
    let persons = sqlx::query_as!(
        Person,
        r#"
SELECT id, last_name, phone_number, location
FROM person;
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(ApiError::DatabaseRequest)?;

    info!("count: {} persons", persons.len());

    Ok(Json(persons))
}
