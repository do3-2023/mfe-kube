mod handlers;
mod models;

use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use handlers::{
    add_message::add_message_handler, get_all_messages::get_all_messages_handler,
    health::health_handler,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

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
    let common_layers = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Micros)),
        )
        .layer(CorsLayer::very_permissive());

    let api_routes = Router::new()
        .route(
            "/messages",
            post(add_message_handler).get(get_all_messages_handler),
        )
        .route("/healthz", get(health_handler));

    Router::new()
        .nest("/api", api_routes)
        .layer(common_layers)
        .with_state(ApiState { pool })
}
