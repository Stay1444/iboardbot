use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    Json,
};
use reqwest::StatusCode;
use tracing::warn;

use crate::api::{
    response::ApiResponse,
    services::boards::{
        entities::{BoardState, Job, JobAction, WriteText},
        Boards,
    },
};

pub async fn action(
    State(boards): State<Boards>,
    Path(id): Path<String>,
) -> (StatusCode, ApiResponse<Job>) {
    if id == "{boardId}" {
        warn!("If you are seeing this its because a board with a boardId of {{boardId}} connected, which means that you did something wrong somewhere.
            If you are sending this from the docs page remember adding a Variable called boardId with your board name.");

        return ApiResponse::bad_req("Skipping {boardId} board");
    }

    let (board, _) = boards.get(id).await;

    match board.state {
        BoardState::Working(job) => ApiResponse::ok(Some(job)),
        _ => ApiResponse::ok(None),
    }
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Get current job for the specified board")
        .summary("Current Job")
        .response_with::<200, Json<ApiResponse<Job>>, _>(|res| {
            res.example(ApiResponse {
                success: true,
                error: None,
                data: Some(Job {
                    id: 62,
                    action: JobAction::WriteText(WriteText {
                        text: "Hello".into(),
                        font: Some("Roboto.ttf".into()),
                    }),
                }),
            })
        })
}
