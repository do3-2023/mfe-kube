mod handlers;
mod models;

use axum::{
    extract::FromRef,
    routing::{delete, get, post},
    Router,
};
use handlers::{
    add_person::add_person_handler, delete_person::delete_person_handler,
    get_all_persons::get_all_persons_handler, health::health_handler,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone, FromRef)]
struct ApiState {
    pub pool: Pool<Postgres>,
}

#[derive(Debug)]
pub enum StartError {
    InvalidBindAddress(std::io::Error),
    ServerNotStarting(std::io::Error),
    CreatePostgresPool(sqlx::Error),
    PostgresMigration(sqlx::migrate::MigrateError),
}

pub async fn run_server(addr: String, pool: Pool<Postgres>) -> Result<(), StartError> {
    let app = create_router(pool);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(StartError::InvalidBindAddress)?;

    axum::serve(listener, app)
        .await
        .map_err(StartError::ServerNotStarting)
}

fn create_router(pool: Pool<Postgres>) -> Router {
    let persons_routes = Router::new()
        .route("/:id", delete(delete_person_handler))
        .route("/", post(add_person_handler).get(get_all_persons_handler));

    let api_routes = Router::new()
        .nest("/persons", persons_routes)
        .route("/healthz", get(health_handler));

    Router::new()
        .nest("/api", api_routes)
        .layer(ServiceBuilder::new().layer(CorsLayer::very_permissive()))
        .with_state(ApiState { pool })
}
