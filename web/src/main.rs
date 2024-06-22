use tracing::info;

#[tokio::main]
async fn main() -> Result<(), mfe_web::StartError> {
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("APP_PORT").unwrap_or("8000".into());
    let addr = format!("{}:{}", host, port);
    info!("listening on http://{}/", addr);

    let worker_api_url = std::env::var("WORKER_API_URL").unwrap_or("localhost:3000".into());

    mfe_web::run_server(addr, worker_api_url).await
}
