use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::rejection::FormRejection;
use reqwest::StatusCode;
use tracing::error;

pub mod pages;
pub mod rest;

#[derive(Debug)]
pub enum WebsiteError {
    AddPersonFormRejection(axum::extract::rejection::FormRejection),
    AddPersonReqwest(reqwest::Error),
    CreatePersonJsonDeserialization(reqwest::Error),
}

#[derive(Debug, Template)]
#[template(path = "components/error.html")]
struct ErrorTemplate {
    message: String,
}

impl IntoResponse for WebsiteError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            WebsiteError::AddPersonFormRejection(e) => {
                error!("AddPersonFormRejection: {:?}", e);
                (e.status(), "Invalid add person form.")
            }
            WebsiteError::AddPersonReqwest(e) => {
                error!("AddPersonReqwest: {:?}", e);
                (StatusCode::BAD_REQUEST, "Could not create the new person.")
            }
            WebsiteError::CreatePersonJsonDeserialization(e) => {
                error!("CreatePersonJsonDeserialization: {:?}", e);
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                (status, "Could not deserialize the new person.")
            }
        };

        let template = ErrorTemplate {
            message: message.into(),
        };

        (status, template).into_response()
    }
}

impl From<FormRejection> for WebsiteError {
    fn from(rejection: FormRejection) -> Self {
        Self::AddPersonFormRejection(rejection)
    }
}
