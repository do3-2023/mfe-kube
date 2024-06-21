use super::WebsiteError;
use crate::{models::Person, Config};
use askama::Template;
use axum::{
    extract::{Path, State},
    Form,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

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

pub async fn delete_person_handler(
    State(config): State<Config>,
    WithRejection(Path(person_id), _): WithRejection<Path<i32>, WebsiteError>,
) -> Result<(), WebsiteError> {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!(
            "http://{}/api/persons/{}",
            config.worker_api_url, person_id
        ))
        .send()
        .await
        .map_err(WebsiteError::DeletePersonReqwest)?;

    if response.status().is_success() {
        return Ok(());
    }

    Err(WebsiteError::CouldNotDeletePerson(response))
}

pub async fn health_handler(State(config): State<Config>) -> Result<String, String> {
    let response = reqwest::get(format!("http://{}/api/healthz", config.worker_api_url))
        .await
        .map_err(|e| {
            error!("could not get health of worker api: {:?}", e);
            String::from("not healthy :'(")
        })?;

    if response.status().is_success() {
        return Ok("healthy <3".into());
    }

    Err(String::from("not healthy :'("))
}
