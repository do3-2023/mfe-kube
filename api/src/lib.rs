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
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct ApiState {
    pub pool: mysql::Pool,
}

#[derive(Debug)]
pub enum StartError {
    InvalidBindAddress(std::io::Error),
    ServerNotStarting,
    CreateDbConnection(mysql::Error),
}

pub async fn run_server(addr: String, pool: mysql::Pool) -> Result<(), StartError> {
    let api_state = ApiState { pool };
    let app = create_router(api_state);

    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
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
