use super::ApiError;
use crate::models::person::{CreatePerson, Person};
use axum::{extract::State, Json};
use axum_extra::extract::WithRejection;
use sqlx::{Pool, Postgres};
use tracing::info;
use validator::Validate;

#[tracing::instrument]
pub async fn add_person_handler(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Json(create_person), _): WithRejection<Json<CreatePerson>, ApiError>,
) -> Result<Json<Person>, ApiError> {
    // check validation schema
    create_person.validate().map_err(ApiError::InvalidRequest)?;

    let inserted_person: Person = sqlx::query_as!(
        Person,
        r#"
INSERT INTO person("last_name", "phone_number", "location")
VALUES($1, $2, $3)
RETURNING id, last_name, phone_number;
        "#,
        create_person.last_name,
        create_person.phone_number,
        create_person.location,
    )
    .fetch_one(&pool)
    .await
    .map_err(ApiError::DatabaseRequest)?;

    info!("new person inserted: {:?}", inserted_person);

    Ok(Json(inserted_person))
}
