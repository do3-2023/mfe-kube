pub mod add_message;
pub mod get_all_messages;
pub mod health;

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum ApiError {
    JsonExtractorRejection(axum::extract::rejection::JsonRejection),
    DatabaseConnection(sqlx::Error),
    DatabaseRequest(sqlx::Error),
    InvalidRequest(validator::ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::JsonExtractorRejection(json_rejection) => {
                error!("{}", json_rejection.body_text());
                (json_rejection.status(), "provided invalid json")
            }
            Self::DatabaseConnection(db_rejection) => {
                error!("{}", db_rejection.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "could not contact the database",
                )
            }
            Self::DatabaseRequest(db_request) => {
                error!("{}", db_request.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error doing the request to the database",
                )
            }
            Self::InvalidRequest(validate_rejection) => {
                error!("{}", validate_rejection.to_string());
                (StatusCode::BAD_REQUEST, "provided invalid request body")
            }
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonExtractorRejection(rejection)
    }
}
