use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("APP_PORT").unwrap_or("3000".into());
    let addr = format!("{}:{}", host, port);
    info!("listening on http://{}/", addr);

    let database_url = std::env::var("DATABASE_URL").expect("no DATABASE_URL given");
    let pool = init_db(&database_url).await.unwrap();

    mfe_api::run_server(addr, pool).await.unwrap();
}

async fn init_db(database_url: &str) -> Result<mysql::Pool, mfe_api::StartError> {
    let pool = mysql::Pool::new(database_url).map_err(mfe_api::StartError::CreateDbConnection)?;

    Ok(pool)
}
