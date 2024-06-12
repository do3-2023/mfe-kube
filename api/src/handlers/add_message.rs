use super::ApiError;
use crate::models::message::{CreateMessage, Message};
use axum::{extract::State, Json};
use axum_extra::extract::WithRejection;
use sqlx::{Pool, Postgres};
use tracing::info;
use validator::Validate;

pub async fn add_message_handler(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Json(create_message), _): WithRejection<Json<CreateMessage>, ApiError>,
) -> Result<Json<Message>, ApiError> {
    // check validation schema
    create_message
        .validate()
        .map_err(ApiError::InvalidRequest)?;

    let inserted_message: Message = sqlx::query_as!(
        Message,
        r#"insert into messages("content") values($1) returning *;"#,
        create_message.content
    )
    .fetch_one(&pool)
    .await
    .map_err(ApiError::DatabaseRequest)?;

    info!("new message inserted: {:?}", inserted_message);

    Ok(Json(inserted_message))
}
