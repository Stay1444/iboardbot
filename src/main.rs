use axum::{
    extract::{FromRef, Query, State},
    response::IntoResponse,
    Router,
};
use serde::Deserialize;
use sessions::Sessions;
use tokio::net::TcpListener;

pub mod sessions;

#[derive(FromRef, Clone)]
struct AppState {
    sessions: Sessions,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    setup_logging();

    let state = AppState {
        sessions: Sessions::new(),
    };

    let service = Router::new()
        .route("/bot", axum::routing::get(bot_action))
        .with_state(state)
        .into_make_service();

    axum::serve(listener, service).await.unwrap()
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

#[derive(Deserialize)]
struct BotActionData {
    #[serde(rename = "APPID", default)]
    app_id: Option<String>,
    #[serde(rename = "STATUS", default)]
    status: Option<BotActionStatus>,
    #[serde(rename = "NUM", default)]
    block_number: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum BotActionStatus {
    Ack,
    Ready,
}

async fn bot_action(
    State(sessions): State<Sessions>,
    Query(data): Query<BotActionData>,
) -> impl IntoResponse {
    let Some(app_id) = data.app_id else {
        return;
    };
    let session = match sessions.get(app_id.clone()).await {
        Some(session) => session,
        None => sessions.create(app_id.clone()).await,
    };
}
