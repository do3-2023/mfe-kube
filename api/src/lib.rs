mod errors;
mod handlers;
mod models;

use crate::handlers::{
    add_message::add_message_handler, get_all_messages::get_all_messages_handler,
};
use anyhow::Context;
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use handlers::health::health_handler;
use sqlx::{Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::info;

#[derive(Debug, Clone, FromRef)]
struct ApiState {
    pub pool: Pool<Postgres>,
}

pub async fn run_server(port: String, pool: Pool<Postgres>) -> anyhow::Result<()> {
    let app = create_app(pool);

    info!("Server listening on port {}", &port);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn create_app(pool: Pool<Postgres>) -> Router {
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
