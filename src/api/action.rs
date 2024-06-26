use std::time::Duration;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use bevy_math::Rect;
use serde::Deserialize;
use tracing::{error, info, warn};

use crate::{
    api::services::boards::entities::JobAction,
    protocol::{BoardAction, BoardMessage},
    utils::{self},
};

use super::services::boards::{entities::SVGSource, Boards};

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

    if id == "{boardId}" {
        warn!("If you are seeing this its because a board with a boardId of {{boardId}} connected, which means that you did something wrong somewhere.
            If you are sending this from the docs page remember adding a Variable called boardId with your board name.");
    }

    let (board, is_new) = boards.get(&id).await;

    if is_new {
        info!("Board {} connected", id);
    }

    let mut message = BoardMessage::new(1);
    let mut job = tokio::select! {
        job = boards.get_job(id.clone()) => {
            job
        },
        _ = tokio::time::sleep(Duration::from_secs(5)) => {
            message.push(BoardAction::Wait(1));
            return message.encode();
        }
    };

    match &mut job.action {
        JobAction::DrawSVG(source) => {
            let svg = match source {
                SVGSource::Raw(svg) => svg.clone(),
                SVGSource::Url(url) => {
                    let Ok(svg) = download_svg(url.clone()).await else {
                        return vec![];
                    };

                    svg
                }
            };

            let Ok((messages, taken)) = utils::svg::draw(
                Rect::new(
                    board.available.0,
                    board.available.1,
                    board.available.2,
                    board.available.3,
                ),
                svg,
            ) else {
                return vec![];
            };

            boards.report_space_taken(&id, taken).await;

            for msg in messages {
                boards.add_job(id.clone(), JobAction::Raw(msg)).await;
            }
        }
        JobAction::WriteText(data) => {
            let (messages, taken) = match utils::text::renderer::render(
                Rect::new(
                    board.available.0,
                    board.available.1,
                    board.available.2,
                    board.available.3,
                ),
                data.text.clone(),
                data.font.clone(),
            ) {
                Ok(x) => x,
                Err(err) => {
                    error!("Error rendering text: {err}");
                    return vec![];
                }
            };
            boards.report_space_taken(&id, taken).await;

            for msg in messages {
                boards.add_job(id.clone(), JobAction::Raw(msg)).await;
            }
        }
        JobAction::Raw(message) => {
            return message.encode();
        }
        JobAction::Calibrate => {
            message.push(BoardAction::StartDrawing);
            message.push(BoardAction::Move(0, 0));
            message.push(BoardAction::PenDown);
            message.push(BoardAction::Move(0, board.details.dimensions.height as u16));
            message.push(BoardAction::Move(
                board.details.dimensions.width as u16,
                board.details.dimensions.height as u16,
            ));
            message.push(BoardAction::Move(board.details.dimensions.width as u16, 0));
            message.push(BoardAction::Move(0, 0));
            message.push(BoardAction::PenUp);
            message.push(BoardAction::StopDrawing);
        }
        JobAction::DrawSVGGroup(sources) => {
            let mut svgs = vec![];
            for source in sources {
                let svg = match source {
                    SVGSource::Raw(svg) => svg.clone(),
                    SVGSource::Url(url) => {
                        let Ok(svg) = download_svg(url.clone()).await else {
                            return vec![];
                        };

                        svg
                    }
                };

                svgs.push(svg);
            }

            let Ok((messages, taken)) = utils::svg::draw_group(
                Rect::new(
                    board.available.0,
                    board.available.1,
                    board.available.2,
                    board.available.3,
                ),
                svgs,
            ) else {
                return vec![];
            };

            boards.report_space_taken(&id, taken).await;

            for msg in messages {
                boards.add_job(id.clone(), JobAction::Raw(msg)).await;
            }
        }
        JobAction::Erase => {
            boards.clear_space(&id).await;
        }
    }

    message.encode()
}

async fn download_svg(url: String) -> anyhow::Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}
