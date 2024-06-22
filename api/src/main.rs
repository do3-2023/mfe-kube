use tracing::{error, info};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("APP_PORT").unwrap_or("3000".into());
    let addr = format!("{}:{}", host, port);
    info!("listening on http://{}/", addr);

    let database_url = std::env::var("DATABASE_URL").expect("no DATABASE_URL given");
    let client = init_db(&database_url).await.unwrap();

    mfe_api::run_server(addr, client).await.unwrap();
}

async fn init_db(database_url: &str) -> Result<tokio_postgres::Client, mfe_api::StartError> {
    let (client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls)
        .await
        .map_err(mfe_api::StartError::CreatePostgresConnection)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("db connection error: {}", e);
        }
    });

    Ok(client)
}
