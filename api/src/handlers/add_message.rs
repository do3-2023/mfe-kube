use crate::{
    errors::ApiError,
    models::message::{CreateMessage, Message},
};
use axum::{extract::State, Json};
use axum_extra::extract::WithRejection;
use sqlx::{Pool, Postgres};
use tracing::info;
use validator::Validate;

pub async fn add_message_handler(
    State(pool): State<Pool<Postgres>>,
    WithRejection(Json(create_message), _): WithRejection<Json<CreateMessage>, ApiError>,
) -> anyhow::Result<Json<Message>, ApiError> {
    // check validation schema
    create_message.validate()?;

    let inserted_message: Message = sqlx::query_as!(
        Message,
        r#"insert into messages("content") values($1) returning *;"#,
        create_message.content
    )
    .fetch_one(&pool)
    .await?;

    info!("new message inserted: {:?}", inserted_message);

    Ok(Json(inserted_message))
}
