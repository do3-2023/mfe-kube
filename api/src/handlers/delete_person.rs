use super::ApiError;
use crate::models::person::Person;
use axum::{
    extract::{Path, State},
    Json,
};
use axum_extra::extract::WithRejection;
use sqlx::{Pool, Postgres};
use tracing::info;

#[tracing::instrument]
pub async fn delete_person_handler(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Path(id), _): WithRejection<Path<i32>, ApiError>,
) -> Result<Json<Person>, ApiError> {
    let deleted_person: Person = sqlx::query_as!(
        Person,
        r#"
DELETE FROM person p
WHERE p.id = $1
RETURNING id, last_name, phone_number;
        "#,
        id,
    )
    .fetch_one(&pool)
    .await
    .map_err(ApiError::DatabaseRequest)?;

    info!("deleted person: {:?}", deleted_person);

    Ok(Json(deleted_person))
}
