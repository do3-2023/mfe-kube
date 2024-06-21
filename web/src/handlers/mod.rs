use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::rejection::FormRejection;
use tracing::error;

pub mod pages;
pub mod rest;

#[derive(Debug)]
pub enum WebsiteError {
    AddPersonFormRejection(axum::extract::rejection::FormRejection),
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
                error!("{}", e);
                (e.status(), "invalid add person form")
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
