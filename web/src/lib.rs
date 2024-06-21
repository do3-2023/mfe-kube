use axum::{
    routing::{delete, get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handlers;
mod models;

#[derive(Debug)]
pub enum StartError {
    InvalidBindAddress(std::io::Error),
    ServerNotStarting(std::io::Error),
}

#[derive(Debug, Clone)]
struct Config {
    pub worker_api_url: String,
}

pub async fn run_server(addr: String, worker_api_url: String) -> Result<(), StartError> {
    let app = create_router(Config { worker_api_url });

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(StartError::InvalidBindAddress)?;

    axum::serve(listener, app)
        .await
        .map_err(StartError::ServerNotStarting)
}

fn create_router(config: Config) -> Router {
    Router::new()
        .route("/", get(handlers::pages::home))
        .route("/healthz", get(handlers::rest::health_handler))
        .route("/api/persons", post(handlers::rest::add_person_handler))
        .route(
            "/api/persons/:id",
            delete(handlers::rest::delete_person_handler),
        )
        .with_state(config)
        .nest_service("/assets", ServeDir::new("dist"))
        .layer(ServiceBuilder::new().layer(CorsLayer::very_permissive()))
}
