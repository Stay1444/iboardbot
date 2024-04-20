use std::sync::Arc;

use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{extract::FromRef, response::Redirect, routing::get, Extension};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::config::Config;

use self::services::boards::Boards;

mod action;
mod controllers;
mod docs;
pub mod services;

#[derive(FromRef, Clone)]
struct AppState {
    boards: Boards,
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let state = AppState {
        boards: Boards::new(),
    };

    aide::gen::on_error(|error| {
        tracing::error!("Aide encountered an error while generating documentation: {error}");
    });

    aide::gen::extract_schemas(true);

    let mut open_api = OpenApi::default();

    let service = ApiRouter::new()
        .nest_api_service("/docs", docs::new())
        .nest_api_service("/api", controllers::routes(state.clone()))
        .route("/_/board/:boardId", get(action::handle))
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .with_state(state)
        .finish_api_with(&mut open_api, cfg_docs)
        .layer(CorsLayer::permissive())
        .layer(Extension(Arc::new(open_api)))
        .into_make_service();

    info!("Listening on 0.0.0.0:{}", config.port);
    info!("API Docs available on http://0.0.0.0:{}/docs", config.port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    axum::serve(listener, service).await?;

    Ok(())
}

fn cfg_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("IBoardBot - Docs")
        .summary("IBoardBot Documentation")
}
