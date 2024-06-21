use crate::models::Person;
use super::WebsiteError;
use askama::Template;
use askama_axum::IntoResponse;
use axum::{debug_handler, Form};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct AddPerson {
    pub last_name: String,
    pub phone_number: String,
    pub locatiodn: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "components/card.html")]
pub struct PersonTemplate {
    pub person: Person,
}

#[debug_handler]
pub async fn add_person_handler(
    WithRejection(Form(add_person), _): WithRejection<Form<AddPerson>, WebsiteError>,
) -> impl IntoResponse {
    info!("new person received: {:?}", add_person);

    // TODO call the backend api
    let new_person = Person {
        _id: Some(0),
        last_name: add_person.last_name,
        phone_number: add_person.phone_number,
        location: add_person.locatiodn,
    };

    PersonTemplate {
        person: new_person,
    }
}
