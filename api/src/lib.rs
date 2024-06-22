mod handlers;
mod models;

use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::{
    add_person::add_person_handler, delete_person::delete_person_handler,
    get_all_persons::get_all_persons_handler, health::health_handler,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ApiState {
    pub postgres_client: Arc<Mutex<tokio_postgres::Client>>,
}

#[derive(Debug)]
pub enum StartError {
    InvalidBindAddressNetStd(std::io::Error),
    InvalidBindAddressNetTokio(std::io::Error),
    ServerNotStarting,
    CreatePostgresConnection(tokio_postgres::Error),
}

pub async fn run_server(addr: String, client: tokio_postgres::Client) -> Result<(), StartError> {
    let api_state = ApiState {
        postgres_client: Arc::new(Mutex::new(client)),
    };
    let app = create_router(api_state);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|_| StartError::ServerNotStarting)
}

fn create_router(state: ApiState) -> Router {
    let persons_routes = Router::new()
        .route("/:id", delete(delete_person_handler))
        .route("/", post(add_person_handler).get(get_all_persons_handler));

    let api_routes = Router::new()
        .nest("/persons", persons_routes)
        .route("/healthz", get(health_handler));

    Router::new().nest("/api", api_routes).with_state(state)
}
