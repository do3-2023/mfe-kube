use crate::{models::Person, Config};
use askama::Template;
use axum::extract::State;
use tracing::error;

#[derive(Debug, Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    persons: Option<Vec<Person>>,
    error_message: Option<String>,
}

pub async fn home(State(config): State<Config>) -> Result<HomeTemplate, HomeTemplate> {
    let persons = reqwest::get(format!("http://{}/api/persons", config.worker_api_url))
        .await
        .map_err(|e| {
            error!("could not fetch: {:?}", e);
            HomeTemplate {
                persons: None,
                error_message: Some("Could not get the persons data.".into()),
            }
        })?
        .json::<Vec<Person>>()
        .await
        .map_err(|e| {
            error!("could not deserialize: {:?}", e);
            HomeTemplate {
                persons: None,
                error_message: Some("Could not get the persons data.".into()),
            }
        })?;

    Ok(HomeTemplate {
        persons: Some(persons),
        error_message: None,
    })
}
