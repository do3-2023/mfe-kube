use super::WebsiteError;
use crate::{models::Person, Config};
use askama::Template;
use axum::{extract::State, Form};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddPerson {
    pub last_name: String,
    pub phone_number: String,
    pub location: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "components/card.html")]
pub struct PersonTemplate {
    pub person: Person,
}

pub async fn add_person_handler(
    State(config): State<Config>,
    WithRejection(Form(add_person), _): WithRejection<Form<AddPerson>, WebsiteError>,
) -> Result<PersonTemplate, WebsiteError> {
    info!("new person received: {:?}", add_person);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://{}/api/persons", config.worker_api_url))
        .json(&add_person)
        .send()
        .await
        .map_err(WebsiteError::AddPersonReqwest)?;

    let new_person = response
        .json::<Person>()
        .await
        .map_err(WebsiteError::CreatePersonJsonDeserialization)?;

    Ok(PersonTemplate { person: new_person })
}
