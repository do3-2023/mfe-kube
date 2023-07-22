use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error("couldn't insert in the database")]
    DatabaseError(#[from] sqlx::Error),
    #[error("invalid request: {0}")]
    InvalidRequest(#[from] ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, mut message) = match self {
            ApiError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
            ApiError::DatabaseError(db_rejection) => {
                (StatusCode::INTERNAL_SERVER_ERROR, db_rejection.to_string())
            }
            ApiError::InvalidRequest(validate_rejection) => {
                (StatusCode::BAD_REQUEST, validate_rejection.to_string())
            }
        };

        // max size for the error message
        message.truncate(128);

        let payload = json!({
            "message": message,
        });

        error!(?status, payload = ?Json(&payload));

        (status, Json(payload)).into_response()
    }
}
