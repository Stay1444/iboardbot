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
        entities::{Job, JobAction, WriteText},
        Boards,
    },
};

pub async fn action(
    State(boards): State<Boards>,
    Path(id): Path<String>,
) -> (StatusCode, ApiResponse<Vec<Job>>) {
    if id == "{boardId}" {
        warn!("If you are seeing this its because a board with a boardId of {{boardId}} connected, which means that you did something wrong somewhere.
            If you are sending this from the docs page remember adding a Variable called boardId with your board name.");

        return ApiResponse::bad_req("Skipping {boardId} board");
    }

    let jobs = boards.list_pending_jobs(id).await;

    ApiResponse::ok(Some(jobs))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("List the pending jobs for the specified board")
        .summary("List Board Jobs")
        .response_with::<200, Json<ApiResponse<Vec<Job>>>, _>(|res| {
            res.example(ApiResponse {
                success: true,
                error: None,
                data: Some(vec![Job {
                    id: 62,
                    action: JobAction::WriteText(WriteText {
                        text: "Hello".into(),
                        font: Some("Roboto.ttf".into()),
                    }),
                }]),
            })
        })
}
