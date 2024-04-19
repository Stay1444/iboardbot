use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::services::boards::{
    entities::{BoardState, Job, JobAction},
    Boards,
};

pub async fn action(State(boards): State<Boards>, Path(id): Path<String>) -> Json<Option<Job>> {
    let board = boards.get(id).await;

    Json(match board.state {
        BoardState::Working(job) => Some(job),
        _ => None,
    })
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Get current job for the specified board")
        .summary("Current Job")
        .response_with::<200, Json<Option<Job>>, _>(|res| {
            res.example(Job {
                id: 62,

                action: JobAction::WriteText("Hello".into()),
            })
        })
}
