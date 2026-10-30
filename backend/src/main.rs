//! Point d'entrée : configuration par variables d'environnement, migrations, import des contenus, API.

mod api;
mod auth;
mod fiches;
mod import;
mod srs;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{path::PathBuf, str::FromStr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/insp.db".into());
    let content_dir =
        PathBuf::from(std::env::var("CONTENT_DIR").unwrap_or_else(|_| "../content".into()));
    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());

    let opts = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(opts).await?;
    sqlx::migrate!().run(&pool).await?;

    let report = import::import_all(&pool, &content_dir).await?;
    tracing::info!(?report, "contenus importés");

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!("API sur {bind}");
    axum::serve(listener, api::router(pool)).await?;
    Ok(())
}
