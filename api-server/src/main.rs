use api_server::{AppState, Config, Result, db};

use axum::{Router, routing::get};
use std::fs;
use std::sync::Arc;
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let mut config = Config::parse()?;

    log::info!("Connecting to database");
    let pool = db::init_pool(config.database(), config.database.pool_size).await?;
    let shared_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/programs", get(db::program::active))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(("localhost", config.http().port)).await?;

    log::info!("Starting server on port {}", config.http().port);
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_logging() -> Result<()> {
    if !fs::exists("logs").unwrap_or(false) {
        fs::create_dir("logs")?;
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Info)
                .chain(std::io::stdout()),
        )
        .chain(fern::DateBased::new("logs/", "%Y-%m-%d_server.log").local_time())
        .apply()?;

    Ok(())
}
