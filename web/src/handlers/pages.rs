use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn home() -> impl IntoResponse {
    HomeTemplate
}
