use clap::Parser;
use tracing::error;

pub mod api;
pub mod config;
pub mod protocol;
pub mod utils;

#[tokio::main]
async fn main() {
    setup_logging();

    let config = config::Config::parse();

    if let Err(err) = api::run(config).await {
        error!("Error running API: {err}");
    }
}

fn setup_logging() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}
