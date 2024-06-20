use tracing::info;

#[tokio::main]
async fn main() -> Result<(), mfe_web::StartError>  {
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("APP_PORT").unwrap_or("8000".into());
    let addr = format!("{}:{}", host, port);
    info!("listening on http://{}/", addr);

    mfe_web::run_server(addr).await
}
