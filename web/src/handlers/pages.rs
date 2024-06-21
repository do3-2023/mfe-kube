use crate::models::Person;
use askama::Template;
use askama_axum::IntoResponse;

#[derive(Debug, Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    persons: Vec<Person>,
}

pub async fn home() -> impl IntoResponse {
    // TODO call the backend api
    let persons = vec![Person {
        _id: Some(0),
        last_name: "John Doe".into(),
        phone_number: "123-4567".into(),
        location: "New York, NY".into(),
    }];

    HomeTemplate { persons }
}
