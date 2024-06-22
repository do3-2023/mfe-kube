use super::ApiError;
use crate::{
    models::person::{CreatePerson, Person},
    ApiState,
};
use axum::{extract::State, Json};
use tracing::info;
use validator::Validate;

pub async fn add_person_handler(
    State(state): State<ApiState>,
    Json(create_person): Json<CreatePerson>,
) -> Result<Json<Person>, ApiError> {
    // check validation schema
    create_person.validate().map_err(ApiError::InvalidRequest)?;

    let row = {
        let pg_client = state.postgres_client.lock().await;

        pg_client
            .query_one(
                r#"
INSERT INTO person("last_name", "phone_number")
VALUES($1::TEXT, $2::TEXT)
RETURNING id, last_name, phone_number;
            "#,
                &[&create_person.last_name, &create_person.phone_number],
            )
            .await
            .map_err(ApiError::DatabaseRequest)?
    };

    let inserted_person = Person {
        id: row.get("id"),
        last_name: row.get("last_name"),
        phone_number: row.get("phone_number"),
    };

    info!("new person inserted: {:?}", inserted_person);

    Ok(Json(inserted_person))
}
