use super::ApiError;
use crate::models::message::Message;
use axum::{extract::State, Json};
use sqlx::{Pool, Postgres};
use tracing::info;

pub async fn get_all_messages_handler(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Message>>, ApiError> {
    let messages = sqlx::query_as!(Message, "select * from messages order by created_at DESC;")
        .fetch_all(&pool)
        .await
        .map_err(ApiError::DatabaseRequest)?;

    info!("count: {} messages", messages.len());

    Ok(Json(messages))
}
