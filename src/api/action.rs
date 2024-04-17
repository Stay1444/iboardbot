use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::protocol::{BoardAction, BoardMessage};

use super::services::boards::Boards;

#[derive(Deserialize, Debug)]
pub struct BotActionData {
    #[serde(rename = "STATUS", default)]
    status: Option<BotActionStatus>,
    #[serde(rename = "NUM", default)]
    block_number: Option<u32>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum BotActionStatus {
    Ack,
    Ready,
}

pub async fn handle(
    Path(id): Path<String>,
    Query(data): Query<BotActionData>,
    State(boards): State<Boards>,
) -> impl IntoResponse {
    if data.status == Some(BotActionStatus::Ack) {
        if data.block_number == Some(0) {
            // Only used for the wait command
            return vec![];
        }

        let Some(block_id) = data.block_number else {
            return vec![];
        };

        boards.ack_job(block_id).await;

        return vec![];
    }

    let Some(job) = boards.pop_job(id).await else {
        let mut message = BoardMessage::new(0);
        message.push(BoardAction::Wait(1));

        return message.encode();
    };

    todo!()
}
