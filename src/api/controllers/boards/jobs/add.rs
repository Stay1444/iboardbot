use aide::transform::TransformOperation;
use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use reqwest::StatusCode;
use tracing::warn;

use crate::api::{
    response::ApiResponse,
    services::boards::{
        entities::{Job, JobAction, SVGSource, WriteText},
        Boards,
    },
};

pub async fn action(
    State(boards): State<Boards>,
    Path(id): Path<String>,
    Json(action): Json<JobAction>,
) -> (StatusCode, ApiResponse<Job>) {
    if id == "{boardId}" {
        warn!("If you are seeing this its because a board with a boardId of {{boardId}} connected, which means that you did something wrong somewhere.
            If you are sending this from the docs page remember adding a Variable called boardId with your board name.");

        return ApiResponse::bad_req("Skipping {boardId} board");
    }

    let job = boards.add_job(id, action).await;
    ApiResponse::ok(Some(job))
}

pub async fn action_multipart(
    State(boards): State<Boards>,
    Path(id): Path<String>,
    mut multipart: Multipart,
) -> (StatusCode, ApiResponse<Job>) {
    if id == "{boardId}" {
        warn!("If you are seeing this its because a board with a boardId of {{boardId}} connected, which means that you did something wrong somewhere.
            If you are sending this from the docs page remember adding a Variable called boardId with your board name.");

        return ApiResponse::bad_req("Skipping {boardId} board");
    }

    while let Some(field) = multipart.next_field().await.unwrap_or_default() {
        let name = field.name().unwrap_or_default().to_string();
        let data = field.text().await.unwrap_or_default();

        if name == "svg" {
            let job = boards
                .add_job(id, JobAction::DrawSVG(SVGSource::Raw(data)))
                .await;

            return ApiResponse::ok(Some(job));
        }
    }

    ApiResponse::bad_req("Invalid file")
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Queue job for the specified board")
        .summary("Queue Job")
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
