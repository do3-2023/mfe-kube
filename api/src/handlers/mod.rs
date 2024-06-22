pub mod add_person;
pub mod delete_person;
pub mod get_all_persons;
pub mod health;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum ApiError {
    DatabaseConnection(tokio_postgres::Error),
    DatabaseRequest(tokio_postgres::Error),
    InvalidRequest(validator::ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::DatabaseConnection(db_rejection) => {
                error!("{}", db_rejection.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "could not reach the database",
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
