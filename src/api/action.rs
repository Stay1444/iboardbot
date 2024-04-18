use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use bevy_math::{Rect, Vec2};
use serde::Deserialize;

use crate::{
    api::services::boards::entities::JobAction,
    protocol::{BoardAction, BoardMessage},
    utils::{self, coords::CoordinateProjector},
};

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
        let Some(block_id) = data.block_number else {
            return vec![];
        };

        boards.ack_job(id, block_id).await;

        return vec![];
    }

    let board = boards.get(&id).await;

    let job = boards.get_job(id.clone()).await;

    let projector = CoordinateProjector::new(Rect::from_corners(
        Vec2::ZERO,
        Vec2::new(
            board.details.dimensions.width as f32,
            board.details.dimensions.height as f32,
        ),
    ));

    let mut message = BoardMessage::new(1);

    match &job.action {
        JobAction::DrawSVG { svg, scale } => {
            let messages = utils::svg::draw(board.details.dimensions, svg.clone(), *scale, false);
            for msg in messages {
                boards.add_job(id.clone(), JobAction::Raw(msg)).await;
            }
        }
        JobAction::WriteLines(lines) => {
            utils::text::write(&mut message, lines.clone(), 400.0, projector, false)
        }
        JobAction::EraseLines(lines) => {
            utils::text::write(&mut message, lines.clone(), 400.0, projector, true)
        }
        JobAction::Raw(message) => {
            return message.encode();
        }
    }

    message.encode()
}
