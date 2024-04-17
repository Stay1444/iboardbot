use std::time::Duration;

use axum::{
    extract::{FromRef, Query, State},
    response::IntoResponse,
    Router,
};
use board::BoardAction;
use serde::Deserialize;
use sessions::Sessions;
use tokio::net::TcpListener;

pub mod board;
pub mod sessions;

#[derive(FromRef, Clone)]
struct AppState {
    sessions: Sessions,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    setup_logging();

    let state = AppState {
        sessions: Sessions::new(),
    };

    let service = Router::new()
        .route("/", axum::routing::get(bot_action))
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

#[derive(Deserialize, Debug)]
struct BotActionData {
    #[serde(rename = "APPID", default)]
    app_id: Option<String>,
    #[serde(rename = "STATUS", default)]
    status: Option<BotActionStatus>,
    #[serde(rename = "NUM", default)]
    block_number: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum BotActionStatus {
    Ack,
    Ready,
}

async fn bot_action(
    State(sessions): State<Sessions>,
    Query(data): Query<BotActionData>,
) -> impl IntoResponse {
    tokio::time::sleep(Duration::from_secs(1)).await;

    match data.status {
        Some(BotActionStatus::Ready) => {
            let commands: Vec<[u8; 3]> = vec![
                BoardAction::StartBlock,
                BoardAction::BlockNumber(1),
                BoardAction::StartDrawing,
                BoardAction::Move(0, 0),
                BoardAction::Wait(2),
                BoardAction::Move(0, 1500),
                BoardAction::Wait(1),
                BoardAction::Eraser,
                BoardAction::Move(0, 0),
                BoardAction::StopDrawing,
            ]
            .into_iter()
            .map(|x| x.serialize())
            .collect();

            let mut data = vec![];

            for i in &commands {
                data.extend_from_slice(i);
            }

            return data;
        }
        _ => return vec![],
    }
}
