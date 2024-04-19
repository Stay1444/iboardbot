use clap::Parser;

pub mod api;
pub mod config;
pub mod protocol;
pub mod sessions;
pub mod utils;

#[tokio::main]
async fn main() {
    setup_logging();

    let config = config::Config::parse();

    api::run(config).await.unwrap();
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
