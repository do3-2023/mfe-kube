use super::ApiError;
use crate::{models::person::CreatePerson, ApiState};
use axum::{extract::State, Json};
use mysql::{params, prelude::Queryable};
use tracing::info;
use validator::Validate;

pub async fn add_person_handler(
    State(state): State<ApiState>,
    Json(create_person): Json<CreatePerson>,
) -> Result<(), ApiError> {
    // check validation schema
    create_person.validate().map_err(ApiError::InvalidRequest)?;

    {
        let mut conn = state
            .pool
            .get_conn()
            .map_err(ApiError::DatabaseConnection)?;

        conn.exec_drop(
            r#"
INSERT INTO person (last_name, phone_number)
VALUES (:last_name, :phone_number);
            "#,
            params! {
                "last_name" => &create_person.last_name,
                "phone_number" => &create_person.phone_number,
            },
        )
        .map_err(ApiError::DatabaseRequest)?
    };

    info!("new person inserted: {:?}", create_person);

    Ok(())
}
