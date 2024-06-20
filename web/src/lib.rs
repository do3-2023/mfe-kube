use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod handlers {
    pub mod pages;
}

#[derive(Debug)]
pub enum StartError {
    InvalidBindAddress(std::io::Error),
    ServerNotStarting(std::io::Error),
}

pub async fn run_server(addr: String) -> Result<(), StartError> {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(StartError::InvalidBindAddress)?;

    axum::serve(listener, app)
        .await
        .map_err(StartError::ServerNotStarting)
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(handlers::pages::home))
        .nest_service("/assets", ServeDir::new("dist"))
        .layer(ServiceBuilder::new().layer(CorsLayer::very_permissive()))
}