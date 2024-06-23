pub mod add_person;
pub mod delete_person;
pub mod get_all_persons;
pub mod health;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum ApiError {
    DatabaseConnection(mysql::Error),
    DatabaseRequest(mysql::Error),
    InvalidRequest(validator::ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::DatabaseConnection(db_rejection) => {
                error!("DatabaseConnection: {:?}", db_rejection.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "could not reach the database",
                )
            }
            Self::DatabaseRequest(db_request) => {
                error!("DatabaseRequest: {:?}", db_request.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error doing the request to the database",
                )
            }
            Self::InvalidRequest(validate_rejection) => {
                error!("InvalidRequest: {:?}", validate_rejection.to_string());
                (StatusCode::BAD_REQUEST, "provided invalid request body")
            }
        };

        let payload = json!({
            "message": message,
        });

        (status, Json(payload)).into_response()
    }
}
